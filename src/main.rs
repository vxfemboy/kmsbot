use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Instant;
use openssl::ssl::{SslMethod, SslConnector, SslStream};


fn main() {

    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect("ircd.chat:6697").unwrap();
    let mut ssl_stream = connector.connect("ircd.chat", stream).unwrap();

    let nick_command = "NICK kmsbot\r\n";
    let user_command = "USER kmsbot 0 * :kmsbot\r\n";
    ssl_stream.write_all(nick_command.as_bytes()).unwrap();
    ssl_stream.write_all(user_command.as_bytes()).unwrap();

    let join_command = "JOIN #channel\r\n";
    ssl_stream.write_all(join_command.as_bytes()).unwrap();

    let mut buf = [0; 512];
    loop {
        match ssl_stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let received = String::from_utf8_lossy(&buf[0..n]);
                let message = received.trim();
                println!("{}", received);

                // Check if the message is %ping
                if message.contains("PRIVMSG #channel :%ping") {
                    let start = Instant::now();
                    let elapsed = start.elapsed();
                    let response = format!("PRIVMSG #channel :Pong: {:?}\r\n", elapsed);
                    ssl_stream.write_all(response.as_bytes()).unwrap();
                }
                

            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            },
        }
    }
}

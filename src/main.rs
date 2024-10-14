use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    if request.starts_with("GET /callback") {
        let response = "HTTP/1.1 200 OK\r\n\r\nAuthorization code received. You can close this window.";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        // Extract the authorization code from the request
        let code = request.split("code=").nth(1).unwrap().split_whitespace().next().unwrap();
        println!("Authorization code: {}", code);

        // Exchange the authorization code for tokens
        exchange_code_for_tokens(code);
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn exchange_code_for_tokens(code: &str) {
    // Manually send an HTTP request to exchange the authorization code for tokens
    // This is a placeholder function. You need to implement the actual HTTP request.
    println!("Exchanging code for tokens: {}", code);
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Server listening on port 8080");

    // Generate the authorization URL
    let client_id = "YOUR_CLIENT_ID";
    let redirect_uri = "http://localhost:8080/callback";
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?response_type=code&client_id={}&redirect_uri={}&scope=openid",
        client_id, redirect_uri
    );
    println!("Open this URL in your browser:\n{}", auth_url);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_client(stream);
        });
    }
}
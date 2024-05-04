use std::collections::HashMap;

fn main() {
    tracing_subscriber::fmt::init();

    println!("Creating connection…");
    let connection = zbus::blocking::Connection::session().unwrap();
    println!("Connection created");

    println!("Calling org.freedesktop.portal.Settings ReadAll method…");
    let input: &[&str] = &["org.freedesktop.appearance"];
    let reply = connection
        .call_method(
            Some("org.freedesktop.portal.Desktop"),
            "/org/freedesktop/portal/desktop",
            Some("org.freedesktop.portal.Settings"),
            "ReadAll",
            &(input),
        )
        .unwrap();
    println!("org.freedesktop.portal.Settings ReadAll method called");

    let output = reply.body();
    let output_struct = output
        .deserialize::<HashMap<String, HashMap<String, zbus::zvariant::Value>>>()
        .unwrap();
    println!("Response: {output_struct:?}");
}

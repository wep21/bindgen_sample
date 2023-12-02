include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        hello();
    }
}

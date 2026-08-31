//! Throughput of the bit-exact `simd` transcendentals vs `wide`'s ~1 ulp
//! forms and the scalar libm calls they replace.
//!
//!     cargo run --release --manifest-path crates/kernels-rayon/math/Cargo.toml \
//!         --example simd_bench
//!
//! Feeds physical-looking magnitudes (log-uniform densities for ln/cbrt,
//! moderate exponents for exp), the same distribution for every contender.

use libxc_rkernel_math::powers;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::f64x8;
use std::time::Instant;

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn f01(&mut self) -> f64 {
        (self.next() >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }
}

const N: usize = 1 << 20;
const REPS: usize = 8;

fn bench(name: &str, xs: &[f64], f: impl Fn(&[f64], &mut [f64])) -> f64 {
    let mut out = vec![0.0f64; xs.len()];
    f(xs, &mut out); // warm
    let mut best = f64::INFINITY;
    for _ in 0..REPS {
        let t = Instant::now();
        f(xs, &mut out);
        let dt = t.elapsed().as_secs_f64();
        best = best.min(dt);
    }
    let ns = best * 1e9 / xs.len() as f64;
    let sum: f64 = out.iter().sum();
    println!("  {name:<28} {ns:7.3} ns/elem   (checksum {sum:.6e})");
    ns
}

fn vec_loop(f: impl Fn(f64x8) -> f64x8) -> impl Fn(&[f64], &mut [f64]) {
    move |xs, out| {
        for (xc, oc) in xs.chunks_exact(8).zip(out.chunks_exact_mut(8)) {
            let mut b = [0.0; 8];
            b.copy_from_slice(xc);
            let r: [f64; 8] = f(f64x8::new(b)).into();
            oc.copy_from_slice(&r);
        }
    }
}

fn cbrt_refined(x: f64x8) -> f64x8 {
    let y = x.cbrt();
    y - (y - x / (y * y)) / f64x8::splat(3.0)
}

fn main() {
    let mut rng = Rng(0x243F6A8885A308D3);
    // ln/cbrt args: log-uniform densities 1e-12..1e4; exp args: uniform -40..12.
    let dens: Vec<f64> = (0..N)
        .map(|_| (rng.f01() * (1e4f64.ln() - 1e-12f64.ln()) + 1e-12f64.ln()).exp())
        .collect();
    let expargs: Vec<f64> = (0..N).map(|_| rng.f01() * 52.0 - 40.0).collect();

    println!("exp ({} elems, best of {REPS}):", N);
    let a = bench("scalar f64::exp", &expargs, |xs, out| {
        for (x, o) in xs.iter().zip(out) {
            *o = x.exp();
        }
    });
    let b = bench("wide .exp()  (~1 ulp)", &expargs, vec_loop(|v| v.exp()));
    let c = bench("simd::exp    (bit-exact)", &expargs, vec_loop(simd::exp));
    println!("  -> bit-exact vs scalar: {:.2}x, vs wide: {:.2}x\n", a / c, b / c);

    println!("ln ({} elems, best of {REPS}):", N);
    let a = bench("scalar f64::ln", &dens, |xs, out| {
        for (x, o) in xs.iter().zip(out) {
            *o = x.ln();
        }
    });
    let b = bench("wide .ln()   (~1 ulp)", &dens, vec_loop(|v| v.ln()));
    let c = bench("simd::ln     (bit-exact)", &dens, vec_loop(simd::ln));
    println!("  -> bit-exact vs scalar: {:.2}x, vs wide: {:.2}x\n", a / c, b / c);

    println!("cbrt ({} elems, best of {REPS}):", N);
    let a = bench("scalar cbrt_f64", &dens, |xs, out| {
        for (x, o) in xs.iter().zip(out) {
            *o = powers::cbrt_f64(*x);
        }
    });
    let b = bench("wide cbrt+Newton (1 ulp)", &dens, vec_loop(cbrt_refined));
    let c = bench("simd::cbrt   (bit-exact)", &dens, vec_loop(simd::cbrt));
    println!("  -> bit-exact vs scalar: {:.2}x, vs wide: {:.2}x\n", a / c, b / c);

    println!("atan ({} elems, best of {REPS}):", N);
    let a = bench("scalar f64::atan", &dens, |xs, out| {
        for (x, o) in xs.iter().zip(out) {
            *o = x.atan();
        }
    });
    let b = bench("wide .atan()  (~1 ulp)", &dens, vec_loop(|v| v.atan()));
    let c = bench("simd::atan   (bit-exact)", &dens, vec_loop(simd::atan));
    println!("  -> bit-exact vs scalar: {:.2}x, vs wide: {:.2}x\n", a / c, b / c);

    println!("tanh ({} elems, best of {REPS}):", N);
    let a = bench("scalar f64::tanh", &expargs, |xs, out| {
        for (x, o) in xs.iter().zip(out) {
            *o = x.tanh();
        }
    });
    let b = bench("wide .tanh()  (~1 ulp)", &expargs, vec_loop(|v| v.tanh()));
    let c = bench("simd::tanh   (bit-exact)", &expargs, vec_loop(simd::tanh));
    println!("  -> bit-exact vs scalar: {:.2}x, vs wide: {:.2}x\n", a / c, b / c);

    println!("erf ({} elems, best of {REPS}):", N);
    let a = bench("scalar rmath::erf", &expargs, |xs, out| {
        for (x, o) in xs.iter().zip(out) {
            *o = rmath::erf(*x);
        }
    });
    let c = bench("simd::erf    (correctly rounded)", &expargs, vec_loop(simd::erf));
    println!("  -> correctly rounded vs scalar: {:.2}x\n", a / c);
}

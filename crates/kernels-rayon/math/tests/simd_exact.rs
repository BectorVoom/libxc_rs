//! Bit-identity of the `simd` transcendentals against the scalar calls they
//! replace: `simd::exp`/`simd::ln` vs the system libm (`f64::exp`/`f64::ln`),
//! `simd::cbrt` and the `pow_n_3` family vs `powers::cbrt_f64` and the scalar
//! `pow_n_3` sequence.
//!
//! These are exact (`to_bits()`) comparisons, not tolerance checks: the whole
//! point of the module is that a SIMD kernel built on it produces the same
//! bits as the scalar kernel. NaN lanes must round-trip the same payload,
//! which holds because non-main-path lanes call the scalar function directly.
//!
//! Run in release (`cargo test --release -p libxc-rkernel-math`) — the sweeps
//! are millions of points.

use libxc_rkernel_math::powers;
// The crate's BitExact `rmath` surface -- the same one the generated scalar
// kernels call. NOT the upstream crate, whose free functions are its Fast path.
use libxc_rkernel_math::rmath;
use libxc_rkernel_math::simd;
use libxc_rkernel_math::wide::f64x8;

/// xorshift64* — deterministic, no dev-dependency.
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
    /// Uniform in [a, b].
    fn uniform(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.f01()
    }
    /// Log-uniform magnitude in [lo, hi], random sign if `signed`.
    fn logmag(&mut self, lo: f64, hi: f64, signed: bool) -> f64 {
        let m = (self.uniform(lo.ln(), hi.ln())).exp();
        if signed && self.next() & 1 == 0 { -m } else { m }
    }
}

fn check_all(vals: &[f64], scalar: impl Fn(f64) -> f64, vector: impl Fn(f64x8) -> f64x8, what: &str) {
    let mut mism = 0usize;
    for chunk in vals.chunks(8) {
        let mut lanes = [chunk[0]; 8];
        lanes[..chunk.len()].copy_from_slice(chunk);
        let got: [f64; 8] = vector(f64x8::new(lanes)).into();
        for l in 0..chunk.len() {
            let want = scalar(lanes[l]);
            if want.to_bits() != got[l].to_bits() {
                mism += 1;
                if mism <= 10 {
                    eprintln!(
                        "{what}: x={:e} ({:#018x}) scalar={:e} ({:#018x}) simd={:e} ({:#018x})",
                        lanes[l],
                        lanes[l].to_bits(),
                        want,
                        want.to_bits(),
                        got[l],
                        got[l].to_bits()
                    );
                }
            }
        }
    }
    assert_eq!(mism, 0, "{what}: {mism} of {} lanes differ", vals.len());
}

/// Edge values every function is fed, plus per-test extras.
fn specials() -> Vec<f64> {
    let mut v = vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::NAN,
        f64::MIN_POSITIVE,
        f64::MIN_POSITIVE / 2.0, // subnormal
        f64::from_bits(1),       // smallest subnormal
        f64::MAX,
        f64::MIN,
    ];
    // Neighborhoods of every branch boundary in exp/ln/cbrt.
    for c in [
        f64::from_bits(0x3c90000000000000), // 2^-54 (exp tiny cut)
        512.0,                              // exp main-path cut
        709.782712893383973096,             // exp overflow cut
        -708.39641853226410622,
        -745.13321910194110842,
        0.9375,             // ln near-1 low cut
        1.06475830078125,   // ln near-1 high cut (1 + 0x1.09p-4)
        f64::MIN_POSITIVE,  // ln normal cut
    ] {
        for d in [-2i64, -1, 0, 1, 2] {
            let b = c.to_bits().wrapping_add(d as u64);
            v.push(f64::from_bits(b));
            v.push(-f64::from_bits(b));
        }
    }
    v
}

#[test]
fn exp_bit_identical_to_rmath() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut vals = specials();
    for _ in 0..1_000_000 {
        vals.push(rng.uniform(-40.0, 40.0)); // physical kernel range
    }
    for _ in 0..500_000 {
        vals.push(rng.uniform(-760.0, 720.0)); // full range incl. under/overflow
    }
    for _ in 0..500_000 {
        vals.push(rng.logmag(1e-30, 1e3, true)); // tiny-|x| branch coverage
    }
    for _ in 0..250_000 {
        vals.push(f64::from_bits(rng.next())); // adversarial bit patterns
    }
    check_all(&vals, rmath::exp, simd::exp, "exp");
}

#[test]
fn ln_bit_identical_to_rmath() {
    let mut rng = Rng(0xD1B54A32D192ED03);
    let mut vals = specials();
    for _ in 0..1_000_000 {
        vals.push(rng.logmag(1e-30, 1e10, false)); // physical densities & co.
    }
    for _ in 0..500_000 {
        vals.push(rng.uniform(0.85, 1.15)); // dense across the near-1 window
    }
    for _ in 0..500_000 {
        vals.push(rng.logmag(1e-320, 1e308, true)); // full range, incl. negatives
    }
    for _ in 0..250_000 {
        vals.push(f64::from_bits(rng.next()));
    }
    check_all(&vals, rmath::ln, simd::ln, "ln");
}

#[test]
fn cbrt_bit_identical_to_scalar_kernels() {
    let mut rng = Rng(0xA0761D6478BD642F);
    let mut vals = specials();
    for _ in 0..1_000_000 {
        vals.push(rng.logmag(1e-30, 1e10, true));
    }
    for _ in 0..500_000 {
        vals.push(f64::from_bits(rng.next()));
    }
    check_all(&vals, powers::cbrt_f64, simd::cbrt, "cbrt");
    check_all(&vals, powers::pow_2_3, simd::pow_2_3, "pow_2_3");
    check_all(&vals, powers::pow_4_3, simd::pow_4_3, "pow_4_3");
    check_all(&vals, powers::pow_5_3, simd::pow_5_3, "pow_5_3");
    check_all(&vals, powers::pow_7_3, simd::pow_7_3, "pow_7_3");
}

#[test]
fn expm1_log1p_bit_identical_to_rmath() {
    let mut rng = Rng(0x55AA55AA33CC33CC);
    let mut vals = specials();
    for _ in 0..100_000 {
        vals.push(rng.uniform(-20.0, 20.0));
    }
    for _ in 0..100_000 {
        vals.push(rng.uniform(-0.999, 10.0)); // log1p domain
    }
    check_all(&vals, rmath::expm1, simd::expm1, "expm1");
    check_all(&vals, rmath::log1p, simd::log1p, "log1p");
}

#[test]
fn atan_tanh_hyperbolics_bit_identical_to_rmath() {
    let mut rng = Rng(0x123456789ABCDEF0);
    let mut vals = specials();
    for _ in 0..100_000 {
        vals.push(rng.uniform(-50.0, 50.0));
    }
    for _ in 0..50_000 {
        vals.push(rng.uniform(-0.999, 0.999)); // atanh domain
    }
    check_all(&vals, rmath::atan, simd::atan, "atan");
    check_all(&vals, rmath::tanh, simd::tanh, "tanh");
    check_all(&vals, rmath::sinh, simd::sinh, "sinh");
    check_all(&vals, rmath::cosh, simd::cosh, "cosh");
    check_all(&vals, rmath::atanh, simd::atanh, "atanh");
}

#[test]
fn trig_and_erf_bit_identical_to_rmath() {
    let mut rng = Rng(0xFEDCBA9876543210);
    let mut vals = specials();
    for _ in 0..100_000 {
        vals.push(rng.uniform(-100.0, 100.0));
    }
    for _ in 0..50_000 {
        vals.push(rng.uniform(-0.999, 0.999)); // asin/acos domain
    }
    check_all(&vals, rmath::sin, simd::sin, "sin");
    check_all(&vals, rmath::cos, simd::cos, "cos");
    check_all(&vals, rmath::tan, simd::tan, "tan");
    check_all(&vals, rmath::erf, simd::erf, "erf");
    check_all(&vals, rmath::erfc, simd::erfc, "erfc");
    check_all(&vals, rmath::asin, simd::asin, "asin");
    check_all(&vals, rmath::acos, simd::acos, "acos");
}

fn check_all2(
    vals1: &[f64],
    vals2: &[f64],
    scalar: impl Fn(f64, f64) -> f64,
    vector: impl Fn(f64x8, f64x8) -> f64x8,
    what: &str,
) {
    let mut mism = 0usize;
    for (chunk1, chunk2) in vals1.chunks(8).zip(vals2.chunks(8)) {
        let mut lanes1 = [chunk1[0]; 8];
        lanes1[..chunk1.len()].copy_from_slice(chunk1);
        let mut lanes2 = [chunk2[0]; 8];
        lanes2[..chunk2.len()].copy_from_slice(chunk2);
        let got: [f64; 8] = vector(f64x8::new(lanes1), f64x8::new(lanes2)).into();
        for l in 0..chunk1.len().min(chunk2.len()) {
            let want = scalar(lanes1[l], lanes2[l]);
            if want.to_bits() != got[l].to_bits() {
                mism += 1;
                if mism <= 10 {
                    eprintln!(
                        "{what}: x={:e}, y={:e} scalar={:e} ({:#018x}) simd={:e} ({:#018x})",
                        lanes1[l],
                        lanes2[l],
                        want,
                        want.to_bits(),
                        got[l],
                        got[l].to_bits()
                    );
                }
            }
        }
    }
    assert_eq!(mism, 0, "{what}: {mism} lanes differ");
}

#[test]
fn binary_exact_functions_bit_identical_to_rmath() {
    let mut rng = Rng(0xCAFEBABE11223344);
    let mut vals1 = specials();
    let mut vals2 = specials();
    for _ in 0..100_000 {
        vals1.push(rng.uniform(0.01, 100.0));
        vals2.push(rng.uniform(-10.0, 10.0));
    }
    check_all2(&vals1, &vals2, rmath::pow, simd::pow, "pow");
    check_all2(&vals1, &vals2, rmath::atan2, simd::atan2, "atan2");
}

/// A special-cased lane must not perturb its neighbours (mask-patch check).
#[test]
fn mixed_special_lanes_do_not_leak() {
    let cases = [
        [1.3, f64::NAN, -2.0, 0.0, 700.0, -745.0, 1e-300, 2.5],
        [f64::INFINITY, 0.9375, 1.0, -0.0, 5e-324, 513.0, -1.0, 1.000001],
    ];
    for lanes in cases {
        let ge: [f64; 8] = simd::exp(f64x8::new(lanes)).into();
        let gl: [f64; 8] = simd::ln(f64x8::new(lanes)).into();
        let gc: [f64; 8] = simd::cbrt(f64x8::new(lanes)).into();
        let ga: [f64; 8] = simd::atan(f64x8::new(lanes)).into();
        let gt: [f64; 8] = simd::tanh(f64x8::new(lanes)).into();
        let gerf: [f64; 8] = simd::erf(f64x8::new(lanes)).into();
        for l in 0..8 {
            assert_eq!(ge[l].to_bits(), rmath::exp(lanes[l]).to_bits(), "exp lane {l}");
            assert_eq!(gl[l].to_bits(), rmath::ln(lanes[l]).to_bits(), "ln lane {l}");
            assert_eq!(gc[l].to_bits(), powers::cbrt_f64(lanes[l]).to_bits(), "cbrt lane {l}");
            assert_eq!(ga[l].to_bits(), rmath::atan(lanes[l]).to_bits(), "atan lane {l}");
            assert_eq!(gt[l].to_bits(), rmath::tanh(lanes[l]).to_bits(), "tanh lane {l}");
            assert_eq!(gerf[l].to_bits(), rmath::erf(lanes[l]).to_bits(), "erf lane {l}");
        }
    }
}

#[test]
fn lambert_w_bit_identical_to_scalar() {
    let mut rng = Rng(0x13579BDF2468ACE0);
    let mut vals = vec![
        -0.3678794411714423, -0.35, -0.3140862435046707, -0.1, -1e-6, 0.0, 1e-6,
        0.5, 1.0, 1.149876485041417, 2.0, 10.0, 100.0, 1e5,
    ];
    for _ in 0..50_000 {
        vals.push(rng.uniform(-0.36, 100.0));
    }
    check_all(&vals, libxc_rkernel_math::lambert_w::lambert_w, simd::lambert_w, "lambert_w");
}


// ---------------------------------------------------------------------------
// rmath vs the platform libm
// ---------------------------------------------------------------------------
//
// Everything above compares `simd::f` (f64x8) against `rmath::f` (scalar).
// That is the right check for what those tests are for -- the SIMD kernel must
// produce the same bits as the scalar kernel, and the scalar kernels call
// `rmath::f` through `from_maple.py`'s LIBM map -- but *both sides go through
// rmath*, so it is self-referential: it cannot detect rmath as a whole drifting
// away from the libm that C libxc calls.
//
// It drifted. Until 2026-08-31 rmath's public free functions were generated
// pinned to the `Fast` policy rather than `BitExact` (`math_fn` &c. in
// rmath's src/function.rs emitted `$Name<Fast, FullRange>`, byte-identical to
// what src/fast.rs generates deliberately). `rmath::ln` *was* `rmath::fast::ln`
// and differed from glibc on 22% of inputs by up to 4 ulp; `atan` on 25% by
// 2 ulp. Every kernel in the tree, scalar and SIMD alike, was running it, and
// every test above still passed, because they compare rmath to rmath.
//
// So this is the test that closes that hole: the thing the 1e-12-vs-libxc
// contract actually depends on is that `rmath::f` equals the platform's `f`.
//
// `erf`/`erfc` are deliberately absent: rmath documents them as *correctly
// rounded*, which is a stronger claim than "bit-identical to glibc" and is
// allowed to differ from a glibc that is not itself correctly rounded.

/// Exact `to_bits()` comparison of a scalar rmath function against the
/// platform libm call that C libxc makes.
fn check_vs_platform(vals: &[f64], rm: impl Fn(f64) -> f64, libm: impl Fn(f64) -> f64, what: &str) {
    let mut mism = 0usize;
    let mut worst = 0i64;
    for &x in vals {
        let (a, b) = (rm(x), libm(x));
        // NaN payloads are IEEE-unspecified; agreeing that it is NaN is enough.
        if a.is_nan() && b.is_nan() {
            continue;
        }
        if a.to_bits() != b.to_bits() {
            mism += 1;
            let ulp = (a.to_bits() as i64).wrapping_sub(b.to_bits() as i64).abs();
            if ulp > worst {
                worst = ulp;
            }
            if mism <= 5 {
                eprintln!(
                    "{what}: x={x:e} rmath={a:e} ({:#018x}) libm={b:e} ({:#018x})",
                    a.to_bits(),
                    b.to_bits()
                );
            }
        }
    }
    assert_eq!(
        mism, 0,
        "{what}: {mism} of {} differ from the platform libm (worst {worst} ulp) \
         -- rmath is not bit-exact here, so the kernels are not either",
        vals.len()
    );
}

#[test]
fn rmath_free_functions_are_bit_exact_against_platform_libm() {
    let mut rng = Rng(0x243F6A8885A308D3);
    let mut wide_vals = specials();
    for _ in 0..200_000 {
        wide_vals.push(rng.logmag(1e-30, 1e10, true));
    }
    // Positive-only domain (ln, cbrt of a positive, pow base).
    let pos: Vec<f64> = wide_vals.iter().map(|v| v.abs()).collect();
    // Bounded domain for the inverse-trig / inverse-hyperbolic family.
    let unit: Vec<f64> = (0..100_000).map(|_| rng.uniform(-0.999, 0.999)).collect();
    // Exponent-safe range.
    let expo: Vec<f64> = (0..100_000).map(|_| rng.uniform(-700.0, 700.0)).collect();

    check_vs_platform(&expo, rmath::exp, f64::exp, "exp");
    check_vs_platform(&pos, rmath::ln, f64::ln, "ln");
    check_vs_platform(&wide_vals, rmath::cbrt, f64::cbrt, "cbrt");
    check_vs_platform(&wide_vals, rmath::atan, f64::atan, "atan");
    check_vs_platform(&expo, rmath::expm1, f64::exp_m1, "expm1");
    check_vs_platform(&unit, rmath::log1p, f64::ln_1p, "log1p");
    check_vs_platform(&wide_vals, rmath::tanh, f64::tanh, "tanh");
    check_vs_platform(&unit, rmath::atanh, f64::atanh, "atanh");
    check_vs_platform(&unit, rmath::asin, f64::asin, "asin");
    check_vs_platform(&unit, rmath::acos, f64::acos, "acos");
    check_vs_platform(&unit, rmath::sin, f64::sin, "sin");
    check_vs_platform(&unit, rmath::cos, f64::cos, "cos");
    check_vs_platform(&unit, rmath::tan, f64::tan, "tan");
    check_vs_platform(&expo, rmath::sinh, f64::sinh, "sinh");
    check_vs_platform(&expo, rmath::cosh, f64::cosh, "cosh");
}

#[test]
fn rmath_binary_free_functions_are_bit_exact_against_platform_libm() {
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut mism = 0usize;
    for _ in 0..200_000 {
        let x = rng.logmag(1e-6, 1e6, false);
        let y = rng.uniform(-10.0, 10.0);
        if rmath::pow(x, y).to_bits() != x.powf(y).to_bits() {
            mism += 1;
        }
        let (a, b) = (rng.logmag(1e-6, 1e6, true), rng.logmag(1e-6, 1e6, true));
        if rmath::atan2(a, b).to_bits() != a.atan2(b).to_bits() {
            mism += 1;
        }
    }
    assert_eq!(mism, 0, "pow/atan2: {mism} results differ from the platform libm");
}

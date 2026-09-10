//! Explicit-SIMD transcendentals backed by `rmath`.
//!
//! Replaces transcendentals for `wide::f64x8` using `rmath` bit-exact/correctly-rounded kernels.

// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

use wide::f64x8;

/// `e^x` for `f64x8`, backed by `rmath::exp`.
#[inline(always)]
pub fn exp(x: f64x8) -> f64x8 {
    rmath::exp(x)
}

/// `ln(x)` for `f64x8`, backed by `rmath::ln`.
#[inline(always)]
pub fn ln(x: f64x8) -> f64x8 {
    rmath::ln(x)
}

/// Cube root for `f64x8`, backed by `rmath::cbrt`.
///
/// `rmath::cbrt`'s vector form (`f64x2`/`f64x4`/`f64x8`) does this same
/// exponent decompose/recompose on genuine `wide::u64xN` integer SIMD
/// rather than a per-lane loop over an array (`~/workspace/rmath`,
/// `src/kernels/double/cbrt.rs`, "Cycle 9" in its ROADMAP.md,
/// 2026-09-07) -- the fix this file's own custom cbrt implementation
/// prototyped earlier the same day, before it was ported upstream.
/// Delegating here keeps one algorithm instead of two independently
/// verified copies; `math/tests/simd_exact.rs::
/// cbrt_bit_identical_to_scalar_kernels` still sweeps this bit-for-bit
/// against `powers::cbrt_f64` and `f64::cbrt`, so a regression in rmath's
/// own vector form would still be caught here, not only upstream.
#[inline(always)]
pub fn cbrt(x: f64x8) -> f64x8 {
    rmath::cbrt(x)
}

/// x^(2/3) for `f64x8` = cbrt(x)^2
#[inline(always)]
pub fn pow_2_3(x: f64x8) -> f64x8 {
    let c = cbrt(x);
    c * c
}

/// x^(4/3) for `f64x8` = x * cbrt(x)
#[inline(always)]
pub fn pow_4_3(x: f64x8) -> f64x8 {
    x * cbrt(x)
}

/// x^(5/3) for `f64x8` = x * cbrt(x)^2
#[inline(always)]
pub fn pow_5_3(x: f64x8) -> f64x8 {
    let c = cbrt(x);
    x * c * c
}

/// x^(7/3) for `f64x8` = x * x * cbrt(x)
#[inline(always)]
pub fn pow_7_3(x: f64x8) -> f64x8 {
    x * x * cbrt(x)
}

/// `e^x - 1` for `f64x8`, backed by `rmath::expm1`.
#[inline(always)]
pub fn expm1(x: f64x8) -> f64x8 {
    rmath::expm1(x)
}

/// `ln(1 + x)` for `f64x8`, backed by `rmath::log1p`.
#[inline(always)]
pub fn log1p(x: f64x8) -> f64x8 {
    rmath::log1p(x)
}

/// `atan(x)` for `f64x8`, backed by `rmath::atan`.
#[inline(always)]
pub fn atan(x: f64x8) -> f64x8 {
    rmath::atan(x)
}

/// `atan2(y, x)` for `f64x8`, backed by `rmath::atan2`.
#[inline(always)]
pub fn atan2(y: f64x8, x: f64x8) -> f64x8 {
    rmath::atan2(y, x)
}

/// `tanh(x)` for `f64x8`, backed by `rmath::tanh`.
#[inline(always)]
pub fn tanh(x: f64x8) -> f64x8 {
    rmath::tanh(x)
}

/// `sinh(x)` for `f64x8`, backed by `rmath::sinh`.
#[inline(always)]
pub fn sinh(x: f64x8) -> f64x8 {
    rmath::sinh(x)
}

/// `cosh(x)` for `f64x8`, backed by `rmath::cosh`.
#[inline(always)]
pub fn cosh(x: f64x8) -> f64x8 {
    rmath::cosh(x)
}

/// `atanh(x)` for `f64x8`, backed by `rmath::atanh`.
#[inline(always)]
pub fn atanh(x: f64x8) -> f64x8 {
    rmath::atanh(x)
}

/// `sin(x)` for `f64x8`, backed by `rmath::sin`.
#[inline(always)]
pub fn sin(x: f64x8) -> f64x8 {
    rmath::sin(x)
}

/// `cos(x)` for `f64x8`, backed by `rmath::cos`.
#[inline(always)]
pub fn cos(x: f64x8) -> f64x8 {
    rmath::cos(x)
}

/// `tan(x)` for `f64x8`, backed by `rmath::tan`.
#[inline(always)]
pub fn tan(x: f64x8) -> f64x8 {
    rmath::tan(x)
}

/// `erf(x)` for `f64x8`, backed by `rmath::erf`.
#[inline(always)]
pub fn erf(x: f64x8) -> f64x8 {
    rmath::erf(x)
}

/// `erfc(x)` for `f64x8`, backed by `rmath::erfc`.
#[inline(always)]
pub fn erfc(x: f64x8) -> f64x8 {
    rmath::erfc(x)
}

/// `pow(x, y)` for `f64x8`, backed by `rmath::pow`.
#[inline(always)]
pub fn pow(x: f64x8, y: f64x8) -> f64x8 {
    rmath::pow(x, y)
}

/// `asin(x)` for `f64x8`, backed by `rmath::asin`.
#[inline(always)]
pub fn asin(x: f64x8) -> f64x8 {
    rmath::asin(x)
}

/// `acos(x)` for `f64x8`, backed by `rmath::acos`.
#[inline(always)]
pub fn acos(x: f64x8) -> f64x8 {
    rmath::acos(x)
}

/// The `dw` of one Halley step of [`lambert_w`], eight lanes, in the scalar's
/// operand order. The caller adds it, because it also needs `dw` for the
/// convergence test.
///
/// `w != -1.0` is an exact comparison, as it is in libxc; an earlier version
/// here used `|w + 1| < 1e-300`, which takes the `dw = 0` branch for a whole
/// neighbourhood the scalar iterates through.
#[inline(always)]
fn halley_step(w: f64x8, z: f64x8) -> f64x8 {
    let one = f64x8::splat(1.0);
    let two = f64x8::splat(2.0);
    let expmw = exp(-w);
    let residual = w - z * expmw;
    let denom = w + one - (w + two) / (two * w + two) * residual;
    let at_pole = w.simd_eq(-one);
    at_pole.select(f64x8::ZERO, -residual / denom)
}

/// Principal branch of the Lambert W function for `f64x8`.
///
/// Bit-identical to [`crate::lambert_w::lambert_w`], including its stopping
/// rule. libxc returns at the first Halley step whose `|dw| < 100 eps (1+|w|)`
/// and returns **0.0** if fifteen steps do not get there; a lane cannot
/// `return`, so a converged lane is *frozen* -- its `w` stops being updated,
/// which is the same thing -- and a lane that never converges is replaced by
/// zero at the end.
#[inline(always)]
pub fn lambert_w(z: f64x8) -> f64x8 {
    const M_E: f64 = std::f64::consts::E;
    let one = f64x8::splat(1.0);

    let inv_e = f64x8::splat(1.0 / M_E);
    let eps = f64x8::splat(f64::EPSILON);
    let cbrt_eps = f64x8::splat(rmath::cbrt(f64::EPSILON));

    let small_res = z - z * z + f64x8::splat(1.5) * z * z * z;

    // Initial guesses. The square root and the logs are evaluated on every
    // lane and selected afterwards, so their arguments are floored to keep a
    // lane that is not going to use them from raising a NaN into the select.
    let branch_arg = (f64x8::splat(2.0 * M_E) * z + f64x8::splat(2.0)).max(f64x8::ZERO);
    let branch_guess = branch_arg.sqrt() - one;
    let pos_z = z.max(f64x8::splat(1e-300));
    let lnz = ln(pos_z);
    let asymp_guess = lnz - ln(lnz.max(f64x8::splat(1e-300)));

    let is_near_branch = z.simd_le(f64x8::splat(-0.3140862435046707));
    let is_taylor = z.simd_le(f64x8::splat(1.149876485041417));
    let mut w = is_near_branch.select(branch_guess, is_taylor.select(small_res, asymp_guess));

    // `done` is the lane-wise stand-in for libxc's `return w`.
    let mut done = f64x8::ZERO.simd_ne(f64x8::ZERO); // all false
    for _ in 0..15 {
        // `halley_step` returns `dw`, not `w + dw`.
        let dw = halley_step(w, z);
        let next = done.select(w, w + dw);
        // The test is applied to the *updated* w, as in the C, where the
        // `return` follows `w += dw`.
        let converged = dw.abs().simd_lt(f64x8::splat(100.0) * eps * (one + next.abs()));
        w = next;
        done |= converged;
    }
    // "This should never happen!" -- libxc warns and returns zero.
    let w = done.select(w, f64x8::ZERO);

    let is_below_branch = z.simd_lt(-inv_e);
    let is_small_z = z.abs().simd_lt(cbrt_eps);
    is_below_branch.select(f64x8::splat(-1.0), is_small_z.select(small_res, w))
}

/// Run a scalar helper on every lane.
///
/// The fallback for the special functions below when a lane is outside the
/// branches they vectorise -- negative, NaN, or (for `erfcx`) beyond the
/// table's range. It is bit-exact by construction: each lane runs the very
/// function the scalar kernel runs, on the very value the scalar kernel would
/// pass. It was the *only* form of `erfcx`/`e1_scaled` until 2026-09-07, and
/// then cost `gga_x_wpbeh vxc` about half its time: 24 scalar calls per
/// 8-point step, each a branchy table or Chebyshev evaluation, inside a loop
/// that was otherwise eight wide.
#[inline(always)]
fn lanewise(x: f64x8, f: fn(f64) -> f64) -> f64x8 {
    let a: [f64; 8] = x.into();
    f64x8::new([f(a[0]), f(a[1]), f(a[2]), f(a[3]), f(a[4]), f(a[5]), f(a[6]), f(a[7])])
}

/// `util.h::xc_cheb_eval` (Clenshaw) on eight lanes, in the scalar's operand
/// order: `b0 = (twox*b1 - b2) + cs[i]`, then `0.5*(b0 - b2)`. Vector mul,
/// sub and add are IEEE per lane and rustc does not contract them into FMAs,
/// so every lane carries the bits `expint_e1::cheb_eval` would produce.
#[inline(always)]
fn cheb_eval(x: f64x8, cs: &[f64]) -> f64x8 {
    let twox = f64x8::splat(2.0) * x;
    let (mut b0, mut b1, mut b2) = (f64x8::ZERO, f64x8::ZERO, f64x8::ZERO);
    for &c in cs.iter().rev() {
        b2 = b1;
        b1 = b0;
        b0 = twox * b1 - b2 + f64x8::splat(c);
    }
    f64x8::splat(0.5) * (b0 - b2)
}

/// libxc's `xc_erfcx` (scaled complementary error function), eight lanes.
///
/// Bit-identical to [`crate::special::xc_erfcx`] on every lane. For `x >= 0`
/// -- which is every call the kernels make, their argument being a `sqrt` --
/// the three arms of the scalar (`x > 5e7`, `x > 50`, and the Faddeeva table
/// for the rest) are all evaluated in vector form and selected per lane. The
/// table arm gathers each lane's seven Chebyshev coefficients from
/// [`crate::erfcx_coef`] by the same `(int) y100` index the scalar `match`
/// dispatches on, then runs the same Horner polynomial. A vector with any
/// negative or NaN lane takes the scalar path for all eight lanes; no
/// generated kernel produces one outside a NaN input.
#[inline(always)]
pub fn erfcx(x: f64x8) -> f64x8 {
    use crate::erfcx_coef::ERFCX_Y100_COEF;
    const ISPI: f64 = 0.56418958354775628694807945156_f64; // 1 / sqrt(pi)

    if !x.simd_ge(f64x8::ZERO).all() {
        return lanewise(x, crate::special::xc_erfcx);
    }

    // Faddeeva table, `y100 = 400/(4+x)` in (0, 100]; `x == 0` lands on
    // `y100 == 100`, which the scalar's `match` sends to its `_ => 1.0` arm.
    let y100 = f64x8::splat(400.0) / (f64x8::splat(4.0) + x);
    let ya: [f64; 8] = y100.into();
    // `as i32` truncates toward zero like C's `(int)` cast. Lanes in the
    // `x > 50` arms have y100 < 7.4 and gather a row they will not use.
    let idx: [usize; 8] = std::array::from_fn(|l| (ya[l] as i32).clamp(0, 100) as usize);
    let row = |l: usize| &ERFCX_Y100_COEF[idx[l].min(99)];
    let coef = |j: usize| f64x8::new([row(0)[j], row(1)[j], row(2)[j], row(3)[j], row(4)[j], row(5)[j], row(6)[j], row(7)[j]]);
    let offs = f64x8::new(std::array::from_fn(|l| (2 * idx[l] + 1) as f64));
    let t = f64x8::splat(2.0) * y100 - offs;
    let table = coef(0)
        + (coef(1) + (coef(2) + (coef(3) + (coef(4) + (coef(5) + coef(6) * t) * t) * t) * t) * t) * t;
    let at_one = f64x8::new(std::array::from_fn(|l| if idx[l] == 100 { 1.0 } else { 0.0 }));
    let table = at_one.simd_eq(f64x8::ONE).select(f64x8::ONE, table);

    let gt50 = x.simd_gt(f64x8::splat(50.0));
    if !gt50.any() {
        return table;
    }
    // Continued-fraction arms, exactly as the scalar spells them.
    let xx = x * x;
    let cf5 = f64x8::splat(ISPI) * (xx * (xx + f64x8::splat(4.5)) + f64x8::splat(2.0))
        / (x * (xx * (xx + f64x8::splat(5.0)) + f64x8::splat(3.75)));
    let cf1 = f64x8::splat(ISPI) / x;
    let big = x.simd_gt(f64x8::splat(5.0e7));
    gt50.select(big.select(cf1, cf5), table)
}

/// libxc's `xc_E1_scaled` (exponentially scaled `E1`), eight lanes.
///
/// Bit-identical to [`crate::expint_e1::xc_e1_scaled`] on every lane. The
/// three positive arms (`x <= 1`, `x <= 4`, `x > 4`) are vectorised, each
/// evaluated only when some lane needs it and selected per lane; a vector
/// with a lane at or below zero, or NaN, takes the scalar path for all
/// eight. The kernels' arguments are sums of squares, so that path is never
/// taken on a finite input. `exp`/`ln` are this module's bit-exact forms,
/// which [`crate::expint_e1::xc_e1_scaled`] also calls (through `rmath`).
#[inline(always)]
pub fn e1_scaled(x: f64x8) -> f64x8 {
    use crate::expint_e1::{AE13, AE14, E12};

    if !x.simd_gt(f64x8::ZERO).all() {
        return lanewise(x, crate::expint_e1::xc_e1_scaled);
    }
    let le1 = x.simd_le(f64x8::ONE);
    let le4 = x.simd_le(f64x8::splat(4.0));
    let s = f64x8::ONE / x;
    let mut r = f64x8::ZERO;
    if !le4.all() {
        // x > 4
        r = s * (f64x8::ONE + cheb_eval(f64x8::splat(8.0) / x - f64x8::ONE, &AE14));
    }
    if le4.any() {
        // 1 < x <= 4
        let mid = s * (f64x8::ONE + cheb_eval((f64x8::splat(8.0) / x - f64x8::splat(5.0)) / f64x8::splat(3.0), &AE13));
        r = le4.select(mid, r);
    }
    if le1.any() {
        // 0 < x <= 1
        let sf = exp(x);
        let lo = sf * (-ln(x.abs()) - f64x8::splat(0.6875) + x + cheb_eval(x, &E12));
        r = le1.select(lo, r);
    }
    r
}

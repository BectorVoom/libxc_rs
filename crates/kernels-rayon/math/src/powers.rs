//! Safe fractional power functions, ported from the CubeCL math crate.
//!
//! Faithful transliteration of `crates/kernels/math/src/powers.rs`: the same
//! expression sequence, the same *truncating* exponent split with the same
//! five-entry rescale table, and `fma` -> `f64::mul_add` (both are a correctly
//! rounded fused multiply-add). Results are therefore bit-identical to the
//! CubeCL kernels, which is what lets the emitted rayon kernels be verified
//! against the CubeCL tree by exact comparison rather than by tolerance.
//!
//! As in libxc's `util.h`, the whole POW_n_3 family is built on a real cube
//! root, not on `pow(x, 1.0/3.0)`:
//!
//! ```c
//! #define POW_1_3(x) cbrt(x)
//! #define POW_2_3(x) (cbrt(x)*cbrt(x))
//! #define POW_4_3(x) ((x)*cbrt(x))
//! #define POW_5_3(x) ((x)*cbrt(x)*cbrt(x))
//! #define POW_7_3(x) ((x)*(x)*cbrt(x))
//! ```

// 2^(k/3) rescaling factors, selected by `xe % 3` in {-2, -1, 0, 1, 2}.
// pub(crate) because `simd::cbrt` replicates this function lanewise and must
// use the identical constants to stay bit-identical.
pub(crate) const CBRT_F_M2: f64 = 0.629960524947436582384; // 2^(-2/3)
pub(crate) const CBRT_F_M1: f64 = 0.793700525984099737376; // 2^(-1/3)
pub(crate) const CBRT_F_P1: f64 = 1.259921049894873164767; // 2^( 1/3)
pub(crate) const CBRT_F_P2: f64 = 1.587401051968199474752; // 2^( 2/3)

/// True cube root of an `f64`, branch-free and with no transcendental call.
///
/// Reduce `x = xm * 2^xe` with `xm` in [0.5, 1), evaluate a degree-6 minimax
/// seed, apply one Halley step, rescale by `2^(xe/3)`, then take one Newton
/// step whose residual `y^3 - x` is formed with an FMA so it survives
/// cancellation. Within 1 ulp of a correctly rounded cube root over the normal
/// range, and exact on exact cubes.
///
/// Handles negatives natively (`cbrt(-8) == -2`) and passes 0, +-inf and NaN
/// through unchanged, matching C's `cbrt`.
#[inline(always)]
pub fn cbrt_f64(x: f64) -> f64 {
    let a = f64::abs(x);
    let bits = a.to_bits();

    // frexp: a == xm * 2^xe with xm in [0.5, 1). Subnormals are scaled by 2^54
    // first, then the exponent is corrected, so they reduce like normals.
    let raw = (bits >> 52) & 0x7ff;
    let is_sub = raw == 0;
    let scaled = a * 18014398509481984.0; // 2^54
    let bits_u = if is_sub { scaled.to_bits() } else { bits };
    let raw_u = (((bits_u >> 52) & 0x7ff) as i32) + if is_sub { -54i32 } else { 0i32 };
    let xm = f64::from_bits((bits_u & 0x800f_ffff_ffff_ffff) | (1022u64 << 52));
    let xe = raw_u - 1022i32;

    // Degree-6 minimax seed for xm^(1/3) on [0.5, 1).
    let u = 0.354895765043919860
        + (1.50819193781584896
            + (-2.11499494167371287
                + (2.44693122563534430
                    + (-1.83469277483613086
                        + (0.784932344976639262 - 0.145263899385486377 * xm) * xm)
                        * xm)
                    * xm)
                * xm)
            * xm;

    // One Halley step, then undo the part of the exponent not divisible by 3.
    let t2 = u * u * u;
    let r = xe % 3i32;
    let fac = if r == -2i32 {
        CBRT_F_M2
    } else if r == -1i32 {
        CBRT_F_M1
    } else if r == 0i32 {
        1.0
    } else if r == 1i32 {
        CBRT_F_P1
    } else {
        CBRT_F_P2
    };
    let ym = u * (t2 + 2.0 * xm) / (2.0 * t2 + xm) * fac;

    // ldexp(ym, xe / 3): `xe / 3` lands in [-358, 341], so the exponent field
    // is always in range and the power of two is exact.
    let n = xe / 3i32;
    let pow2 = f64::from_bits(((n + 1023i32) as u64) << 52);
    let y0 = ym * pow2;

    // Newton polish. The FMA keeps the residual y^3 - a accurate; computing it
    // as y*y*y - a would lose every significant digit to cancellation.
    let t = y0 * y0;
    let err = f64::mul_add(t, y0, -a);
    let y = y0 - err / (3.0 * t);

    let signed = if x < 0.0 { -y } else { y };
    let degenerate = (x == 0.0) || x.is_nan() || x.is_infinite();
    if degenerate { x + x } else { signed }
}

/// Cube root, handling negatives correctly. libxc `CBRT(x)` / `POW_1_3(x)`.
#[inline(always)]
pub fn safe_cbrt(x: f64) -> f64 {
    cbrt_f64(x)
}

/// x^(1/3)
#[inline(always)]
pub fn pow_1_3(x: f64) -> f64 {
    safe_cbrt(x)
}

/// x^(2/3) = cbrt(x)^2
#[inline(always)]
pub fn pow_2_3(x: f64) -> f64 {
    let c = safe_cbrt(x);
    c * c
}

/// x^(4/3) = x * cbrt(x)
#[inline(always)]
pub fn pow_4_3(x: f64) -> f64 {
    x * safe_cbrt(x)
}

/// x^(5/3) = x * cbrt(x)^2
#[inline(always)]
pub fn pow_5_3(x: f64) -> f64 {
    let c = safe_cbrt(x);
    x * c * c
}

/// x^(7/3) = x * x * cbrt(x)
#[inline(always)]
pub fn pow_7_3(x: f64) -> f64 {
    x * x * safe_cbrt(x)
}

/// x^(3/2) = x * sqrt(x)
#[inline(always)]
pub fn pow_3_2(x: f64) -> f64 {
    x * f64::sqrt(x)
}

/// x^(1/4) = sqrt(sqrt(x))
#[inline(always)]
pub fn pow_1_4(x: f64) -> f64 {
    f64::sqrt(f64::sqrt(x))
}

/// x^2
#[inline(always)]
pub fn pow_2(x: f64) -> f64 {
    x * x
}

/// x^3
#[inline(always)]
pub fn pow_3(x: f64) -> f64 {
    x * x * x
}

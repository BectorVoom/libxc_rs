//! The `rmath` surface the generated kernels call, pinned to `BitExact`.
//!
//! # Why this module exists
//!
//! `rmath`'s top-level free functions -- `rmath::exp`, `rmath::ln`, … -- are
//! **deliberately** the `Fast` path. That is rmath's design, documented on each
//! function ("fast approximation safe on any input") and asserted by its own
//! `tests/fast_path.rs::test_toplevel_free_functions_use_fast_path`, which
//! requires `rmath::exp(x) == rmath::fast::exp(x)`. Bit-exactness there is
//! opt-in, through the function objects: `Exp::<BitExact, FullRange>`.
//!
//! This tree took the fast path by accident. `from_maple.py`'s `LIBM` map sends
//! every scalar kernel's `exp`/`log`/`atan`/… to `rmath::exp`/`ln`/`atan`, and
//! `simd.py`'s `FREE_EXACT` sends every SIMD kernel's to
//! `libxc_rkernel_math::simd::*`, which forwarded to the same free functions.
//! So both kernel forms agreed with *each other* -- fingerprints stayed put,
//! and `tests/simd_exact.rs` passed because it compares rmath against rmath --
//! while the tree as a whole had drifted from the libm C libxc calls.
//!
//! Measured against glibc over 200k inputs in 1e-8..1e8, before this module:
//!
//! | fn     | differing | worst |
//! |--------|-----------|-------|
//! | `ln`   | 22.24 %   | 4 ulp |
//! | `atan` | 24.86 %   | 2 ulp |
//! | `exp`  | 10.56 %   | 1 ulp |
//! | `cbrt` | 8.47 %    | 1 ulp |
//! | `sqrt` | 0 %       | 0 ulp |
//!
//! Against a 1e-12-vs-oracle contract that is a real exposure, because a
//! derivative expression amplifies an input ulp by orders of magnitude -- the
//! tree's own record has 1 ulp of `exp` reaching 4.4e-12 in `gga_c_lyp`, and
//! `rmath::policy::Fast`'s rustdoc warns about exactly this.
//!
//! # How it works
//!
//! `lib.rs` exports this module *as* `rmath`, so the generated kernels' own
//! `use libxc_rkernel_math::rmath;` picks it up unchanged: no regeneration, no
//! edit to the `LIBM` map, and no way for a kernel to reach the fast path by
//! writing the obvious thing. Everything this module does not name is
//! re-exported from the real crate by the glob below, so `rmath::fast` (the
//! explicit opt-in) and the policy types stay reachable under the same path.
//!
//! One generic function serves both kernel forms: `Function::eval` is generic
//! over `V: Simd<Elem = f64>`, and rmath implements `Simd` for plain `f64` as
//! well as for `wide::f64x8`. So `rmath::exp` is bit-exact whether the caller
//! is a scalar kernel or a `f64x8` one, which is what makes the two forms
//! bit-identical to each other *and* to the platform.

pub use ::rmath_upstream::*;

use ::rmath_upstream::prelude::{
    Acos, Acosh, Asin, Asinh, Atan, Atan2, Atanh, BitExact, Cos, Cosh, Erf, Erfc, Exp,
    Expm1, Fmax, Fmin, Function, Function2, FullRange, Hypot, Ln, Log1p, Log2, Log10, Pow, Simd,
    Sin, Sinh, Tan, Tanh,
};

/// Define a `BitExact`-pinned unary function shadowing rmath's `Fast` one.
macro_rules! exact1 {
    ($(#[$doc:meta])* $name:ident, $Obj:ident) => {
        $(#[$doc])*
        #[inline(always)]
        pub fn $name<V: Simd<Elem = f64>>(x: V) -> V {
            <$Obj<BitExact, FullRange> as Function<f64>>::eval(&$Obj::default(), x)
        }
    };
}

/// Define a `BitExact`-pinned binary function shadowing rmath's `Fast` one.
macro_rules! exact2 {
    ($(#[$doc:meta])* $name:ident, $Obj:ident) => {
        $(#[$doc])*
        #[inline(always)]
        pub fn $name<V: Simd<Elem = f64>>(x: V, y: V) -> V {
            <$Obj<BitExact, FullRange> as Function2<f64>>::eval(&$Obj::default(), x, y)
        }
    };
}

// Every transcendental `from_maple.py`'s LIBM map or `simd.py`'s FREE_EXACT
// can emit. `sqrt`/`abs`/`fmin`/`fmax` are in rmath's "Exact" group, where the
// two policies run the same code, but they are pinned here too so that no
// reader has to know which group a given name falls in.
exact1!(/// `e^x`, bit-exact against the platform libm.
        exp, Exp);
exact1!(/// `ln(x)`, bit-exact against the platform libm.
        ln, Ln);
exact1!(/// `log2(x)`, bit-exact against the platform libm.
        log2, Log2);
exact1!(/// `log10(x)`, bit-exact against the platform libm.
        log10, Log10);
exact1!(/// `e^x - 1`, bit-exact against the platform libm.
        expm1, Expm1);
exact1!(/// `ln(1 + x)`, bit-exact against the platform libm.
        log1p, Log1p);
exact1!(/// `sin(x)`, bit-exact against the platform libm.
        sin, Sin);
exact1!(/// `cos(x)`, bit-exact against the platform libm.
        cos, Cos);
exact1!(/// `tan(x)`, bit-exact against the platform libm.
        tan, Tan);
exact1!(/// `asin(x)`, bit-exact against the platform libm.
        asin, Asin);
exact1!(/// `acos(x)`, bit-exact against the platform libm.
        acos, Acos);
exact1!(/// `atan(x)`, bit-exact against the platform libm.
        atan, Atan);
exact1!(/// `sinh(x)`, bit-exact against the platform libm.
        sinh, Sinh);
exact1!(/// `cosh(x)`, bit-exact against the platform libm.
        cosh, Cosh);
exact1!(/// `tanh(x)`, bit-exact against the platform libm.
        tanh, Tanh);
exact1!(/// `asinh(x)`, bit-exact against the platform libm.
        asinh, Asinh);
exact1!(/// `acosh(x)`, bit-exact against the platform libm.
        acosh, Acosh);
exact1!(/// `atanh(x)`, bit-exact against the platform libm.
        atanh, Atanh);
// `erf`/`erfc` are rmath's "correctly rounded" group: `BitExact` returns the
// nearest representable value, which is a stronger guarantee than matching a
// glibc that is not itself correctly rounded. Pinned for the same reason as
// the rest -- the caller should not silently get the cheaper form.
exact1!(/// `erf(x)`, correctly rounded.
        erf, Erf);
exact1!(/// `erfc(x)`, correctly rounded.
        erfc, Erfc);

exact2!(/// `x^y`, bit-exact against the platform libm.
        pow, Pow);
exact2!(/// `atan2(y, x)`, bit-exact against the platform libm.
        atan2, Atan2);
exact2!(/// `hypot(x, y)`, bit-exact against the platform libm.
        hypot, Hypot);
exact2!(/// `fmin(x, y)`, bit-exact against the platform libm.
        fmin, Fmin);
exact2!(/// `fmax(x, y)`, bit-exact against the platform libm.
        fmax, Fmax);


// ---------------------------------------------------------------------------
// `cbrt`: glibc's algorithm, not rmath's.
// ---------------------------------------------------------------------------
//
// Every other function above pins rmath's `BitExact` policy, and for those that
// policy reproduces glibc bit for bit (measured against glibc resolved through
// `dlopen("libm.so.6")`: 0 of 1,000,000 differ for `ln`, `exp`, `atan`, `erf`,
// `erfc`, `pow`). **`cbrt` is the exception, by rmath's own design**:
// `rmath::reference::double::cbrt` is documented as "bit-identical to Rust's
// `f64::cbrt`, not the C library's" -- Rust's std ports core-math's correctly
// rounded `cbrt` rather than calling libm -- and it warns that glibc's cruder
// algorithm "disagrees on roughly half of a random sweep". Measured: 566,113 of
// 1,000,000 inputs in 1e-12..1e4, one ulp each.
//
// Why no gate caught it: every comparison this tree ran against "libm cbrt"
// was made inside a Rust binary, where the symbol `cbrt` is satisfied by
// `compiler_builtins::math::libm_math::cbrt` -- a static definition, which the
// linker prefers over the one in `libm.so.6`. `nm` on the `kernel_oracle` and
// `composite_oracle` binaries shows `cbrt` defined locally and NOT imported,
// while `log`/`exp`/`atan`/`erf`/`erfc`/`pow` are imported from glibc as
// expected. So the vendored C libxc in those oracles was calling Rust's
// `cbrt`, `rmath::cbrt` agreed with it, and both disagreed with the glibc
// `cbrt` that PySCF's libxc (`U cbrt@GLIBC_2.2.5`) actually calls. The same
// trap is why `examples/cbrt_check.rs`'s premise -- "`f64::cbrt` on Linux *is*
// that libm" -- does not hold.
//
// The trade-off is real and deliberate: glibc's `cbrt` is about 1 ulp
// accurate, rmath's is correctly rounded. The C library this tree is gated
// against calls glibc's, so reproducing libxc means reproducing glibc here,
// and doing it with a less accurate cube root. Bit-exactness against a C
// libxc is also inherently a property of the C library it links: on a
// platform whose libm implements `cbrt` differently, this is the wrong
// function to match.
//
// Proven, not argued: `tests/cbrt_glibc_parity.rs` sweeps `cbrt` (scalar and
// `f64x8`) against glibc's own symbol obtained by `dlopen`/`dlsym`, which the
// static linker cannot substitute.

/// `2^(1/3)`, glibc's `CBRT2`.
const GLIBC_CBRT2: f64 = 1.2599210498948731648;
/// `2^(2/3)`, glibc's `SQR_CBRT2`.
const GLIBC_SQR_CBRT2: f64 = 1.5874010519681994748;
/// glibc's `factor[5]`, indexed by `2 + xe % 3`.
const GLIBC_CBRT_FACTOR: [f64; 5] = [
    1.0 / GLIBC_SQR_CBRT2,
    1.0 / GLIBC_CBRT2,
    1.0,
    GLIBC_CBRT2,
    GLIBC_SQR_CBRT2,
];

/// `x^(1/3)`, bit-identical to glibc 2.43's `cbrt` --
/// `sysdeps/ieee754/dbl-64/s_cbrt.c`, ported operation for operation, which
/// is what x86_64 uses (there is no arch-specific override). See the section
/// comment above for why this does not use rmath's `cbrt`.
///
/// Every `+`, `*` and `/` below is written in glibc's order and none may be
/// fused: glibc's baseline x86_64 build has no FMA, and rustc does not
/// contract. `frexp` and `ldexp` are done on the bits and are exact: `xm` is
/// renormalised into `[0.5, 1)` (subnormals via an exact `2^54` pre-scale, as
/// glibc's own `frexp` does), and the `2^(xe/3)` scale is an exact power of
/// two whose product can neither overflow nor underflow for any finite input.
#[inline(always)]
pub fn cbrt_glibc(x: f64) -> f64 {
    let ax = x.abs();
    let bits = ax.to_bits();
    let e = ((bits >> 52) & 0x7ff) as i32;
    // glibc: `if (xe == 0 && fpclassify (x) <= FP_ZERO) return x + x;` --
    // exactly zero, infinity and NaN.
    if e == 0x7ff || ax == 0.0 {
        return x + x;
    }
    // `xm = __frexp (fabs (x), &xe);`
    const MANT: u64 = 0x000f_ffff_ffff_ffff;
    let (xm, xe) = if e == 0 {
        let s = ax * f64::from_bits(0x4350_0000_0000_0000); // 2^54, exact
        let sb = s.to_bits();
        let se = ((sb >> 52) & 0x7ff) as i32;
        (f64::from_bits((sb & MANT) | (1022u64 << 52)), se - 1022 - 54)
    } else {
        (f64::from_bits((bits & MANT) | (1022u64 << 52)), e - 1022)
    };

    let u = 0.354895765043919860
        + ((1.50819193781584896
            + ((-2.11499494167371287
                + ((2.44693122563534430
                    + ((-1.83469277483613086
                        + (0.784932344976639262 - 0.145263899385486377 * xm) * xm)
                        * xm))
                    * xm))
                * xm))
            * xm);

    let t2 = u * u * u;

    // C's `%` and `/` truncate toward zero, as Rust's do, so a negative `xe`
    // indexes and scales identically.
    let ym = u * (t2 + 2.0 * xm) / (2.0 * t2 + xm) * GLIBC_CBRT_FACTOR[(2 + xe % 3) as usize];

    // `__ldexp (x > 0.0 ? ym : -ym, xe / 3)`: |xe / 3| <= 358, so `2^(xe/3)`
    // is a normal double and the product is exact.
    let scale = f64::from_bits(((1023 + xe / 3) as u64) << 52);
    (if x > 0.0 { ym } else { -ym }) * scale
}

/// `x^(1/3)`, bit-exact against glibc's `cbrt` -- the one C libxc calls.
///
/// Generic like the rest of this module, so scalar and `f64x8` kernels stay
/// bit-identical to each other and to glibc: each lane goes through
/// [`cbrt_glibc`]. This gives up the lane-parallel exponent surgery rmath's
/// own vector `cbrt` did; correctness first, and the per-lane scalar port is
/// the oracle any future vectorised form must reproduce.
#[inline(always)]
pub fn cbrt<V: Simd<Elem = f64>>(x: V) -> V {
    use ::rmath_upstream::simd::Lanes;
    let mut a = x.to_array();
    for v in a.as_mut_slice() {
        *v = cbrt_glibc(*v);
    }
    V::from_array(a)
}

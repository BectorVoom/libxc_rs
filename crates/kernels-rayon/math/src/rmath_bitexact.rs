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
    Acos, Acosh, Asin, Asinh, Atan, Atan2, Atanh, BitExact, Cbrt, Cos, Cosh, Erf, Erfc, Exp,
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
exact1!(/// `x^(1/3)`, bit-exact against the platform libm.
        cbrt, Cbrt);
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

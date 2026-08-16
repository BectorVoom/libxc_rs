//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 707/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk707(t123: f64, t3689: f64, t883: f64, t912: f64, t587: f64, t2488: f64, t2487: f64, t12079: f64, t901: f64, t2366: f64, t2365: f64, t1429: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13777 = t3689 * t123;
    let t13778 = t13777 * t883;
    let t13779 = t912 * t13778;
    let t13780 = t587 * t13779;
    let t13782 = t2488 * t13778;
    let t13783 = t2487 * t13782;
    let t13789 = t12079 * t901;
    let t13791 = t2366 * t3689;
    let t13792 = t2365 * t13791;
    let t13793 = t1429 * t13792;
    (t13778, t13779, t13780, t13782, t13783, t13789, t13791, t13792, t13793)
}

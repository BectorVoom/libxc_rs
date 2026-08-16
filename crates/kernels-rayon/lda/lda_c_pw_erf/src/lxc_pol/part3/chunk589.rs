//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 589/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk589(t1653: f64, t2061: f64, t1953: f64, t432: f64, t416: f64, t1124: f64, t118: f64, t119: f64, t120: f64, t1687: f64, t1568: f64, t437: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3280 = 1.2991222222222223_f64 * t1653 * t2061;
    let t3282 = 0.7617244444444444_f64 * t432 * t1953;
    let t3284 = 1.5156425925925925_f64 * t416 * t1953;
    let t3288 = 7.0_f64 / 27.0_f64 * t118 * t119 * t1124 * t120;
    let t3290 = 0.6529066666666666_f64 * t1687 * t2061;
    let t3291 = t437 * t1568;
    (t3280, t3282, t3284, t3288, t3290, t3291)
}

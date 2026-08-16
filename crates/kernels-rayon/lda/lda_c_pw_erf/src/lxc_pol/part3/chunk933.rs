//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 933/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk933(t1453: f64, t3783: f64, t519: f64, t1458: f64, t155: f64, t1461: f64, t3723: f64, t3883: f64, t1446: f64, t3880: f64, t3884: f64, t3788: f64, t3794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10311 = t519 * t3783 * t1453;
    let t10313 = t155 * t1458;
    let t10315 = t519 * t10313 * t1461;
    let t10318 = t519 * t3883 * t3723;
    let t10320 = t1446 * t3880;
    let t10322 = t1446 * t3884;
    let t10326 = t3794 * t3788;
    (t10311, t10313, t10315, t10318, t10320, t10322, t10326)
}

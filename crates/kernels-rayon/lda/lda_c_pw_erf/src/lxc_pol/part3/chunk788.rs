//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 788/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk788(t549: f64, t833: f64, t593: f64, t5269: f64, t1318: f64, t2005: f64, t945: f64, t1326: f64, t1325: f64, t1319: f64, t4684: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5270 = t833 * t549;
    let t5271 = t5270 * t593;
    let t5272 = t5269 * t5271;
    let t5274 = 16.0_f64 / 15.0_f64 * t1318 * t5272;
    let t5275 = t2005 * t945;
    let t5276 = t1326 * t5275;
    let t5278 = 8.0_f64 / 45.0_f64 * t1325 * t5276;
    let t5279 = t1319 * t4684;
    let t5281 = 8.0_f64 / 15.0_f64 * t571 * t5279;
    (t5270, t5271, t5272, t5274, t5275, t5276, t5278, t5279, t5281)
}

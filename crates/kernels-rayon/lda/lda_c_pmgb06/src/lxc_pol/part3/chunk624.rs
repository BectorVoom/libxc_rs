//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 624/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk624(t3568: f64, t3576: f64, t1243: f64, t3572: f64, t342: f64, t409: f64, t55: f64, t1276: f64, t110: f64, t1263: f64, t360: f64, t1234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3577 = t3576 * t3568;
    let t3578 = 0.9743416666666667_f64 * t3577;
    let t3579 = t1243 * t3572;
    let t3580 = 1.4615125_f64 * t3579;
    let t3582 = t55 * t409 * t342;
    let t3583 = t1276 * t3582;
    let t3585 = t110 * t1263;
    let t3586 = t360 * t3585;
    let t3588 = t1234 * t342;
    (t3577, t3578, t3579, t3580, t3582, t3583, t3585, t3586, t3588)
}

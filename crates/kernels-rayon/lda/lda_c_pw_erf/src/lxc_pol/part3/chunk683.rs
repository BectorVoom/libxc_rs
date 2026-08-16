//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 683/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk683(t474: f64, t603: f64, t602: f64, t1: f64, t1112: f64, t3: f64, t604: f64, t1631: f64, t1635: f64, t1422: f64, t20: f64, t1639: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4183 = t474 * t603;
    let t4185 = 0.09618703433213194_f64 * t602 * t4183;
    let t4187 = t1112 * t1 * t3;
    let t4188 = t4187 * t604;
    let t4190 = t1631 * t1635;
    let t4192 = t1422 * t20;
    let t4193 = t4192 * t1639;
    (t4183, t4185, t4187, t4188, t4190, t4192, t4193)
}

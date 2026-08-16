//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1097/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1097(t15947: f64, t1924: f64, t493: f64, t1925: f64, t6134: f64, t432: f64, t7863: f64, t161: f64, t489: f64, t7725: f64, t16583: f64, t531: f64, t7628: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20197 = t493 * t15947 * t1924 / 15.0_f64;
    let t20199 = t6134 * t1925 / 15.0_f64;
    let t20201 = t432 * t7863 / 10.0_f64;
    let t20203 = t161 * t489 * t7725;
    let t20204 = 2.0_f64 / 15.0_f64 * t20203;
    let t20205 = 2.0_f64 / 15.0_f64 * t16583;
    let t20207 = t7628 * t531 / 30.0_f64;
    (t20197, t20199, t20201, t20204, t20205, t20207)
}

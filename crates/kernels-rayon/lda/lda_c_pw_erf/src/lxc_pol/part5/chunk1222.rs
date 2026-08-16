//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1222/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1222(t12874: f64, t1308: f64, t1325: f64, t1326: f64, t13551: f64, t15614: f64, t16121: f64, t18133: f64, t2146: f64, t2385: f64, t34: f64, t3794: f64, t4738: f64, t4763: f64, t4804: f64, t4829: f64, t4841: f64, t571: f64, t6256: f64, t6263: f64, t6285: f64, t6357: f64, t6401: f64, t6455: f64, t739: f64, t743: f64, t7809: f64) -> f64 {
    let t22082 = 16.0_f64 / 15.0_f64 * t2146 * t6401 - 4.0_f64 / 15.0_f64 * t571 * t1308 * t16121 * t743 + 8.0_f64 / 15.0_f64 * t571 * t4841 * t6285 * t34 + 16.0_f64 / 15.0_f64 * t15614 * t2385 + 16.0_f64 / 15.0_f64 * t12874 * t2385 + 32.0_f64 / 15.0_f64 * t4738 * t6455 + 8.0_f64 / 15.0_f64 * t4804 * t7809 + 8.0_f64 / 15.0_f64 * t3794 * t7809 + 8.0_f64 / 15.0_f64 * t1325 * t1326 * t18133 * t739 + 16.0_f64 / 15.0_f64 * t1325 * t4829 * t6263 * t34 + t13551 + 8.0_f64 / 15.0_f64 * t4763 * t6256 - 4.0_f64 / 15.0_f64 * t2146 * t6357;
    t22082
}

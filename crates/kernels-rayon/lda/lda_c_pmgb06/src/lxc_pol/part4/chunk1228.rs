//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1228/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1228(t12112: f64, t4836: f64, t802: f64, t4830: f64, t1554: f64, t161: f64, t2600: f64, t132: f64, t435: f64, t6583: f64, t6571: f64, t16145: f64, t16149: f64, t16151: f64, t16153: f64, t16157: f64, t16159: f64, t16162: f64, t16167: f64, t16171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16172 = 4.0_f64 / 135.0_f64 * t12112;
    let t16173 = t802 * t4836;
    let t16174 = 2.0_f64 / 135.0_f64 * t16173;
    let t16176 = 4.0_f64 / 45.0_f64 * t802 * t4830;
    let t16178 = t161 * t1554 * t2600;
    let t16179 = 2.0_f64 / 135.0_f64 * t16178;
    let t16181 = t132 * t435 * t6583;
    let t16182 = 2.0_f64 / 45.0_f64 * t16181;
    let t16184 = t132 * t435 * t6571;
    let t16185 = 4.0_f64 / 45.0_f64 * t16184;
    let t16186 = -t16145 + t16149 - t16151 - t16153 + t16157 - t16159 - t16162 + t16167 - t16171 + t16172 + t16174 + t16176 - t16179 - t16182 - t16185;
    (t16172, t16174, t16176, t16179, t16182, t16185, t16186)
}

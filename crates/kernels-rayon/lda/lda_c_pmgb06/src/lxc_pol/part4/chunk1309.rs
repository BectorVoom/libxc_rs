//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1309/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1309(t405: f64, t6891: f64, t4913: f64, t6894: f64, t6897: f64, t6900: f64, t103: f64, t15503: f64, t1576: f64, t16073: f64, t16354: f64, t17143: f64, t17147: f64, t17152: f64, t17160: f64, t17188: f64, t3358: f64, t525: f64) -> f64 {
    let t17215 = t405 * t6891;
    let t17217 = t4913 * t6894;
    let t17222 = t405 * t6897;
    let t17224 = t405 * t6900;
    let t17229 = 0.02666666666666667_f64 * t103 * t525 * t15503 + 0.013333333333333334_f64 * t103 * t525 * t17160 + 0.013333333333333334_f64 * t103 * t1576 * t17188 - 0.0044444444444444444_f64 * t103 * t1576 * t17143 - 0.0022222222222222222_f64 * t103 * t1576 * t17147 - 0.002962962962962963_f64 * t103 * t3358 * t17152 + 0.05333333333333334_f64 * t17215 + 0.2311111111111111_f64 * t17217 - 0.04_f64 * t103 * t525 * t16073 - 0.017777777777777778_f64 * t17222 + 0.002962962962962963_f64 * t17224 + 0.013333333333333334_f64 * t103 * t1576 * t16354;
    t17229
}

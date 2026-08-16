//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 769/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk769(t5086: f64, t5094: f64, t5077: f64, t4095: f64, t4097: f64, t4099: f64, t4102: f64, t4105: f64, t4106: f64, t4108: f64, t4115: f64, t4117: f64, t4121: f64, t5064: f64, t5074: f64, t5081: f64, t5089: f64, t5093: f64) -> (f64, f64, f64) {
    let t5095 = t5094 * t5086;
    let t5097 = 4.0_f64 / 45.0_f64 * t5077 * t5095;
    let t5098 = 2.0_f64 / 3.0_f64 * t4095 + 0.2431111111111111_f64 * t4097 - 4.0_f64 / 27.0_f64 * t4099 - t4102 + t4105 + 2.0_f64 / 9.0_f64 * t4106 + 8.0_f64 / 9.0_f64 * t4108 + t4115 + t4117 - t4121 - t5064 + t5074 + t5081 - t5089 + t5093 + t5097;
    (t5095, t5097, t5098)
}

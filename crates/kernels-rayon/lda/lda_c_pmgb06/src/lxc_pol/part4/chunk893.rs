//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 893/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk893(t5049: f64, t5052: f64, t2606: f64, t432: f64, t3007: f64, t4070: f64, t4079: f64, t4082: f64, t4095: f64, t4097: f64, t4099: f64, t4102: f64, t4105: f64, t4108: f64, t4115: f64, t4117: f64, t4121: f64) -> (f64, f64, f64, f64) {
    let t6433 = 2.0_f64 / 135.0_f64 * t5049;
    let t6434 = 2.0_f64 / 135.0_f64 * t5052;
    let t6440 = t432 * t2606 / 15.0_f64;
    let t6441 = -t6433 - t6434 + t3007 + t4070 + t4079 + t4082 + t4095 / 3.0_f64 + 0.12155555555555556_f64 * t4097 - 2.0_f64 / 27.0_f64 * t4099 - t4102 + t4105 + 4.0_f64 / 9.0_f64 * t4108 + t4115 + t4117 - t4121 + t6440;
    (t6433, t6434, t6440, t6441)
}

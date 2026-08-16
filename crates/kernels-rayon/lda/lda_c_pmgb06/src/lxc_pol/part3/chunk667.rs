//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 667/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk667(t1698: f64, t591: f64, t1701: f64, t4111: f64, t208: f64, t315: f64, t586: f64, t584: f64, t1691: f64, t213: f64, t4079: f64, t4082: f64, t4084: f64, t4089: f64, t4091: f64, t4095: f64, t4097: f64, t4099: f64, t4102: f64, t4105: f64, t4106: f64, t4108: f64) -> (f64, f64, f64, f64, f64) {
    let t4115 = 2.0_f64 / 3.0_f64 * t1698 * t591;
    let t4117 = (2e-21_f64 as f64) * t1701 * t4111;
    let t4119 = t586 * t315 * t208;
    let t4121 = 0.013506172839506173_f64 * t584 * t4119;
    let t4122 = t4079 + t4082 + t4084 * t213 / 3.0_f64 + t4089 + 0.18233333333333332_f64 * t4091 + t4095 + 0.36466666666666664_f64 * t4097 - 2.0_f64 / 9.0_f64 * t4099 - t4102 + t4105 + 2.0_f64 / 3.0_f64 * t4106 + 4.0_f64 / 3.0_f64 * t4108 + (2e-21_f64 as f64) * t1691 * t4111 + t4115 + t4117 - t4121;
    (t4115, t4117, t4119, t4121, t4122)
}

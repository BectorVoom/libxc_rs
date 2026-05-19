//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 667/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk667<F: Float>(t1698: F, t591: F, t1701: F, t4111: F, t208: F, t315: F, t586: F, t584: F, t1691: F, t213: F, t4079: F, t4082: F, t4084: F, t4089: F, t4091: F, t4095: F, t4097: F, t4099: F, t4102: F, t4105: F, t4106: F, t4108: F) -> (F, F, F, F, F) {
    let t4115 = F::new(2.0) / F::new(3.0) * t1698 * t591;
    let t4117 = F::new(2e-21) * t1701 * t4111;
    let t4119 = t586 * t315 * t208;
    let t4121 = F::cast_from(0.013506172839506173_f64) * t584 * t4119;
    let t4122 = t4079 + t4082 + t4084 * t213 / F::new(3.0) + t4089 + F::cast_from(0.18233333333333332_f64) * t4091 + t4095 + F::cast_from(0.36466666666666664_f64) * t4097 - F::new(2.0) / F::new(9.0) * t4099 - t4102 + t4105 + F::new(2.0) / F::new(3.0) * t4106 + F::new(4.0) / F::new(3.0) * t4108 + F::new(2e-21) * t1691 * t4111 + t4115 + t4117 - t4121;
    (t4115, t4117, t4119, t4121, t4122)
}

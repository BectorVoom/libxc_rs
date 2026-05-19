//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 893/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk893<F: Float>(t5049: F, t5052: F, t2606: F, t432: F, t3007: F, t4070: F, t4079: F, t4082: F, t4095: F, t4097: F, t4099: F, t4102: F, t4105: F, t4108: F, t4115: F, t4117: F, t4121: F) -> (F, F, F, F) {
    let t6433 = F::new(2.0) / F::new(135.0) * t5049;
    let t6434 = F::new(2.0) / F::new(135.0) * t5052;
    let t6440 = t432 * t2606 / F::new(15.0);
    let t6441 = -t6433 - t6434 + t3007 + t4070 + t4079 + t4082 + t4095 / F::new(3.0) + F::cast_from(0.12155555555555556_f64) * t4097 - F::new(2.0) / F::new(27.0) * t4099 - t4102 + t4105 + F::new(4.0) / F::new(9.0) * t4108 + t4115 + t4117 - t4121 + t6440;
    (t6433, t6434, t6440, t6441)
}

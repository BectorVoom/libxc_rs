//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 753/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk753<F: Float>(t1381: F, t5090: F, t5068: F, t1531: F, t465: F, t5086: F, t5077: F, t4095: F, t4097: F, t4099: F, t4102: F, t4105: F, t4106: F, t4108: F, t4115: F, t4117: F, t4121: F, t5064: F, t5074: F, t5081: F, t5089: F) -> (F, F, F, F, F, F) {
    let t5091 = t5090 * t1381;
    let t5093 = F::new(4.0) / F::new(45.0) * t5068 * t5091;
    let t5094 = t465 * t1531;
    let t5095 = t5094 * t5086;
    let t5097 = F::new(4.0) / F::new(45.0) * t5077 * t5095;
    let t5098 = F::new(2.0) / F::new(3.0) * t4095 + F::cast_from(0.2431111111111111_f64) * t4097 - F::new(4.0) / F::new(27.0) * t4099 - t4102 + t4105 + F::new(2.0) / F::new(9.0) * t4106 + F::new(8.0) / F::new(9.0) * t4108 + t4115 + t4117 - t4121 - t5064 + t5074 + t5081 - t5089 + t5093 + t5097;
    (t5091, t5093, t5094, t5095, t5097, t5098)
}

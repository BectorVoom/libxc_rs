//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 808/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk808<F: Float>(t1179: F, t4068: F, t580: F, t1147: F, t206: F, t208: F, t31: F, t99: F, t213: F, t398: F, t4075: F, t4641: F, t4913: F, t83: F, t4076: F, t588: F, t97: F) -> (F, F, F, F, F) {
    let t9461 = 0.006061752703703704 * t580 * t1179 * t4068;
    let t9467 = 0.0002763148940771605 * t206 * t1147 * t99 * t31 * t208;
    let t9470 = t398 * t4075 * t208 * t213;
    let t9478 = t83 * (-0.33530864197530863 * t4641 + 1.8360493827160493 * t4913) * t208 * t213 / 3.0;
    let t9481 = 0.2431111111111111 * t4076 * t97 * t588;
    (t9461, t9467, t9470, t9478, t9481)
}

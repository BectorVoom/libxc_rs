//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 894/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk894<F: Float>(t208: F, t213: F, t398: F, t4075: F, t4641: F, t4913: F, t83: F, t4076: F, t588: F, t97: F, t4093: F, t2803: F, t579: F) -> (F, F, F, F, F) {
    let t9470 = t398 * t4075 * t208 * t213;
    let t9478 = t83 * (-F::cast_from(0.33530864197530863_f64) * t4641 + F::cast_from(1.8360493827160493_f64) * t4913) * t208 * t213 / F::cast_from(3.0_f64);
    let t9481 = F::cast_from(0.2431111111111111_f64) * t4076 * t97 * t588;
    let t9483 = t4093 * t97 * t588;
    let t9491 = t2803 * t579 * t208 * t213;
    (t9470, t9478, t9481, t9483, t9491)
}

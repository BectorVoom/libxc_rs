//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1177/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1177<F: Float>(t5044: F, t831: F, t11777: F, t9242: F, t9259: F, t5302: F, t802: F, t9267: F, t9269: F, t11792: F, t12261: F, t161: F, t166: F, t851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15467 = t831 * t5044;
    let t15468 = F::new(2.0) / F::new(135.0) * t15467;
    let t15469 = F::new(8.0) / F::new(405.0) * t11777;
    let t15470 = t9242 / F::new(135.0);
    let t15471 = t9259 / F::new(135.0);
    let t15472 = t802 * t5302;
    let t15473 = F::new(4.0) / F::new(45.0) * t15472;
    let t15474 = F::new(4.0) / F::new(405.0) * t9267;
    let t15475 = F::new(4.0) / F::new(405.0) * t9269;
    let t15476 = F::new(4.0) / F::new(45.0) * t11792;
    let t15480 = t161 * t166 * t12261 * t851 / F::new(15.0);
    (t15468, t15469, t15470, t15471, t15473, t15474, t15475, t15476, t15480)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 580/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk580<F: Float>(t3115: F, t443: F, t453: F, t36: F, t3081: F, t3082: F, t3084: F, t3086: F, t3088: F, t3095: F, t3101: F, t3106: F, t3110: F, t3113: F) -> (F, F, F, F) {
    let t3116 = t443 * t3115;
    let t3117 = t453 * t3116;
    let t3118 = t36 * t3117;
    let t3120 = t3081 + F::new(0.002518888888888889) * t3082 - F::new(0.0012594444444444445) * t3084 + F::new(0.003778333333333333) * t3086 - F::new(0.0018891666666666666) * t3088 + F::new(0.002099074074074074) * t3095 - F::new(0.007556666666666666) * t3101 + F::new(0.003778333333333333) * t3106 + F::new(0.011335) * t3110 - F::new(0.011335) * t3113 + F::new(0.0018891666666666666) * t3118;
    (t3116, t3117, t3118, t3120)
}

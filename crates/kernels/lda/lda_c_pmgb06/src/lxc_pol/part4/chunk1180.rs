//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1180/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1180<F: Float>(t11830: F, t11832: F, t12006: F, t2088: F, t493: F, t529: F, t851: F, t5264: F, t6275: F, t1423: F, t6491: F, t6495: F) -> (F, F, F, F, F, F) {
    let t15510 = F::new(8.0) / F::new(45.0) * t11830;
    let t15511 = F::new(8.0) / F::new(135.0) * t11832;
    let t15516 = F::new(4.0) / F::new(5.0) * t493 * t12006 * t851 * t529 * t2088;
    let t15518 = F::new(8.0) / F::new(27.0) * t6275 * t5264;
    let t15519 = t1423 * t6491;
    let t15520 = F::new(8.0) / F::new(135.0) * t15519;
    let t15521 = t1423 * t6495;
    (t15510, t15511, t15516, t15518, t15520, t15521)
}

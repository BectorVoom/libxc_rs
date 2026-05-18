//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1246/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1246<F: Float>(t12772: F, t1907: F, t439: F, t12603: F, t12621: F, t12623: F, t12625: F, t12631: F, t13100: F, t493: F, t834: F, t2462: F, t3198: F) -> (F, F, F, F, F, F, F, F) {
    let t16411 = F::new(4.0) / F::new(45.0) * t439 * t12772 * t1907;
    let t16412 = F::new(8.0) / F::new(135.0) * t12603;
    let t16413 = F::new(8.0) / F::new(405.0) * t12621;
    let t16414 = F::new(8.0) / F::new(135.0) * t12623;
    let t16415 = F::new(8.0) / F::new(135.0) * t12625;
    let t16416 = F::new(8.0) / F::new(135.0) * t12631;
    let t16419 = F::new(2.0) / F::new(45.0) * t493 * t13100 * t834;
    let t16421 = F::new(2.0) / F::new(45.0) * t3198 * t2462;
    (t16411, t16412, t16413, t16414, t16415, t16416, t16419, t16421)
}

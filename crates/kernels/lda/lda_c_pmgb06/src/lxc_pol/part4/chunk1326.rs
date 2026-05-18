//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1326/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1326<F: Float>(t1083: F, t6560: F, t5138: F, t5139: F, t12529: F, t12530: F, t16386: F, t13312: F, t15264: F, t337: F, t12537: F, t11904: F, t6562: F) -> (F, F, F, F, F, F, F) {
    let t17427 = t6560 * t1083;
    let t17430 = F::new(2.0) / F::new(27.0) * t5138 * t5139 * t17427;
    let t17433 = F::new(16.0) / F::new(81.0) * t12529 * t12530 * t16386;
    let t17434 = F::new(16.0) / F::new(135.0) * t13312;
    let t17435 = t15264 * t337;
    let t17438 = F::new(8.0) / F::new(27.0) * t12537 * t5139 * t17435;
    let t17440 = F::new(8.0) / F::new(45.0) * t11904 * t6562;
    (t17427, t17430, t17433, t17434, t17435, t17438, t17440)
}

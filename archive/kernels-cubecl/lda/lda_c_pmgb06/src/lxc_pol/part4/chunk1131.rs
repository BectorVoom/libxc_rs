//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1131/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1131<F: Float>(t123: F, t199: F, t315: F, t4463: F, t4454: F, t566: F, t4259: F, t868: F, t1156: F, t1808: F, t2771: F, t4351: F) -> (F, F, F, F, F) {
    let t14723 = t123 * t315 * t4463 * t199;
    let t14726 = t123 * t4454 * t566;
    let t14741 = t123 * t4259 * t868;
    let t14744 = t123 * t1156 * t1808;
    let t14758 = t4351 * t2771;
    (t14723, t14726, t14741, t14744, t14758)
}

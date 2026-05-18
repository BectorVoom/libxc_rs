//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1125/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1125<F: Float>(t1347: F, t1799: F, t117: F, t123: F, t315: F, t5689: F, t1795: F, t118: F, t5575: F, t2174: F, t415: F, t14239: F) -> (F, F, F, F, F, F) {
    let t14535 = t1799 * t1347;
    let t14539 = t123 * t315 * t5689 * t117;
    let t14541 = t1795 * t1347;
    let t14543 = t5575 * t118;
    let t14545 = t2174 * t415;
    let t14547 = t14239 * t118;
    (t14535, t14539, t14541, t14543, t14545, t14547)
}

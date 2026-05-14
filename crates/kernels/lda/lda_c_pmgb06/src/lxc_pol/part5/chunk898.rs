//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 898/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk898<F: Float>(t6976: F, t947: F, t110: F, t6979: F, t360: F, t2703: F, t410: F, t6967: F, t6970: F, t377: F, t7041: F, t1295: F, t2718: F, t2247: F, t5858: F, t7081: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18749 = t6976 * t947;
    let t18751 = t110 * t6979;
    let t18752 = t360 * t18751;
    let t18754 = t410 * t2703;
    let t18755 = t360 * t18754;
    let t18757 = t6967 * t947;
    let t18759 = t6970 * t947;
    let t18796 = t7041 * t377;
    let t18807 = t2718 * t1295;
    let t18815 = t2247 * t5858 * t7081;
    (t18749, t18751, t18752, t18754, t18755, t18757, t18759, t18796, t18807, t18815)
}

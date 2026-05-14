//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 688/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk688<F: Float>(t1869: F, t4641: F, t3092: F, t760: F, t1069: F, t3090: F, t36: F, t3098: F, t1525: F, t1: F, t1438: F, t332: F, t1830: F, t1074: F, t1858: F, t453: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4642 = t4641 * t1869;
    let t4644 = t3092 * t760;
    let t4645 = t4644 * t1069;
    let t4646 = t3090 * t4645;
    let t4647 = t36 * t4646;
    let t4649 = t3098 * t760;
    let t4650 = t4649 * t1069;
    let t4651 = t1525 * t4650;
    let t4652 = t36 * t4651;
    let t4654 = t1438 * t1;
    let t4655 = t4654 * t332;
    let t4656 = t1525 * t4655;
    let t4657 = t1830 * t4656;
    let t4659 = t1858 * t1074;
    let t4660 = t1525 * t4659;
    let t4661 = t36 * t4660;
    let t4663 = t1858 * t1069;
    let t4664 = t453 * t4663;
    (t4642, t4645, t4646, t4647, t4650, t4651, t4652, t4655, t4656, t4657, t4659, t4660, t4661, t4663, t4664)
}

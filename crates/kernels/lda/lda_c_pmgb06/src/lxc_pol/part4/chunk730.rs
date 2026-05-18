//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 730/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk730<F: Float>(t1525: F, t4655: F, t1830: F, t1074: F, t1858: F, t36: F, t1069: F, t453: F, t1: F, t1531: F, t332: F, t1863: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4656 = t1525 * t4655;
    let t4657 = t1830 * t4656;
    let t4659 = t1858 * t1074;
    let t4660 = t1525 * t4659;
    let t4661 = t36 * t4660;
    let t4663 = t1858 * t1069;
    let t4664 = t453 * t4663;
    let t4665 = t36 * t4664;
    let t4667 = t1531 * t1;
    let t4668 = t4667 * t332;
    let t4669 = t453 * t4668;
    let t4670 = t1830 * t4669;
    let t4672 = t1863 * t1074;
    (t4656, t4657, t4659, t4660, t4661, t4663, t4664, t4665, t4668, t4669, t4670, t4672)
}

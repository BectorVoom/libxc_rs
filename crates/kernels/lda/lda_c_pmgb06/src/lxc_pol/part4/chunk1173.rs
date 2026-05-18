//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1173/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1173<F: Float>(t30: F, t32: F, t8083: F, t1438: F, t79: F, t1525: F, t1531: F, t453: F, t350: F, t6221: F, t1830: F, t1863: F) -> (F, F, F, F, F, F, F) {
    let t15426 = t30 * t32 * t8083;
    let t15427 = t1438 * t79;
    let t15429 = t15426 * t1525 * t15427;
    let t15431 = t1531 * t79;
    let t15433 = t15426 * t453 * t15431;
    let t15435 = t350 * t6221;
    let t15438 = t1830 * t453 * t1863;
    (t15426, t15427, t15429, t15431, t15433, t15435, t15438)
}

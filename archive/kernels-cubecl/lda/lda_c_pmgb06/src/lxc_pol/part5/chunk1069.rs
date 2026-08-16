//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1069/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1069<F: Float>(t1830: F, t19475: F, t3090: F, t1531: F, t7290: F, t332: F, t36: F, t453: F, t1525: F, t19490: F, t12396: F, t12406: F, t19618: F) -> (F, F, F, F, F) {
    let t19799 = t1830 * t3090 * t19475;
    let t19801 = t1531 * t7290;
    let t19802 = t19801 * t332;
    let t19804 = t36 * t453 * t19802;
    let t19807 = t36 * t1525 * t19490;
    let t19811 = t12396 * t12406 * t19618;
    (t19799, t19802, t19804, t19807, t19811)
}

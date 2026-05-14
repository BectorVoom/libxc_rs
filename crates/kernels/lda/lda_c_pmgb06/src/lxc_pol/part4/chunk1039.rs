//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1039/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1039<F: Float>(t1525: F, t15426: F, t15427: F, t1531: F, t79: F, t453: F, t350: F, t6221: F, t1830: F, t1863: F, t1069: F, t6150: F, t36: F, t332: F, t5961: F, t12393: F, t15413: F, t15416: F, t15418: F, t15421: F, t15423: F, t9178: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15429 = t15426 * t1525 * t15427;
    let t15431 = t1531 * t79;
    let t15433 = t15426 * t453 * t15431;
    let t15435 = t350 * t6221;
    let t15438 = t1830 * t453 * t1863;
    let t15440 = t6150 * t1069;
    let t15442 = t36 * t1525 * t15440;
    let t15445 = t1531 * t5961 * t332;
    let t15447 = t36 * t453 * t15445;
    let t15449 = 0.002099074074074074 * t15413 + 0.005037777777777778 * t12393 - t9178 + 0.0008396296296296296 * t15416 + 0.000559753086419753 * t15418 + 0.005037777777777778 * t15421 - 0.0016792592592592592 * t15423 + 0.010075555555555556 * t15429 - 0.030226666666666666 * t15433 - 0.0012594444444444445 * t15435 - 0.015113333333333333 * t15438 + 0.04534 * t15442 - 0.007556666666666666 * t15447;
    (t15429, t15431, t15433, t15435, t15438, t15440, t15442, t15445, t15447, t15449)
}

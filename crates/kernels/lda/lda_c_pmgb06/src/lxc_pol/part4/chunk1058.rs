//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1058/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1058<F: Float>(t9350: F, t9352: F, t9379: F, t9381: F, t11897: F, t161: F, t489: F, t6231: F, t5110: F, t831: F, t1069: F, t1438: F, t2648: F, t2960: F, t439: F, t2002: F, t5268: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15801 = 8.0 / 405.0 * t9350;
    let t15802 = 8.0 / 405.0 * t9352;
    let t15803 = 2.0 / 405.0 * t9379;
    let t15804 = 2.0 / 243.0 * t9381;
    let t15805 = 4.0 / 135.0 * t11897;
    let t15807 = t161 * t489 * t6231;
    let t15808 = 2.0 / 45.0 * t15807;
    let t15810 = 2.0 / 15.0 * t831 * t5110;
    let t15815 = t439 * t2960 * t2648 * t1438 * t1069 / 27.0;
    let t15817 = 2.0 / 45.0 * t2002 * t5268;
    (t15801, t15802, t15803, t15804, t15805, t15808, t15810, t15815, t15817)
}

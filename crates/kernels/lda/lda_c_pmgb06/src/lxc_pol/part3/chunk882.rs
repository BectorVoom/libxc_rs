//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 882/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk882<F: Float>(t11792: F, t486: F, t4941: F, t1730: F, t2025: F, t2021: F, t3209: F, t439: F, t6550: F, t2002: F, t3191: F, t9291: F, t9293: F, t9295: F, t9297: F, t1179: F, t4068: F, t871: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11793 = 2.0 / 15.0 * t11792;
    let t11795 = t486 * t4941 / 5.0;
    let t11796 = t2025 * t1730;
    let t11798 = t2021 * t1730;
    let t11799 = 0.09973633333333333 * t11798;
    let t11802 = t439 * t6550 * t3209 / 5.0;
    let t11804 = 2.0 / 9.0 * t2002 * t3191;
    let t11805 = 2.0 / 45.0 * t9291;
    let t11806 = 4.0 / 45.0 * t9293;
    let t11807 = 2.0 / 27.0 * t9295;
    let t11808 = 2.0 / 27.0 * t9297;
    let t11810 = t871 * t1179 * t4068;
    (t11793, t11795, t11796, t11799, t11802, t11804, t11805, t11806, t11807, t11808, t11810)
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 748/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk748<F: Float>(t1962: F, t2492: F, t439: F, t1972: F, t2489: F, t1988: F, t2488: F, t493: F, t3172: F, t7295: F, t1462: F, t2465: F, t1439: F, t7284: F, t442: F, t1465: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7651 = t1962 * t2492;
    let t7653 = 2.0 / 15.0 * t439 * t7651;
    let t7655 = 2.0 / 15.0 * t1972 * t2489;
    let t7656 = t1988 * t2488;
    let t7658 = 2.0 / 15.0 * t493 * t7656;
    let t7659 = t3172 * t7295;
    let t7660 = t1462 * t7659;
    let t7662 = 2.0 / 9.0 * t493 * t7660;
    let t7663 = t1988 * t2465;
    let t7665 = t493 * t7663 / 15.0;
    let t7666 = t1439 * t7284;
    let t7667 = t442 * t7666;
    let t7669 = 2.0 / 15.0 * t439 * t7667;
    let t7670 = t1465 * t7295;
    (t7651, t7653, t7655, t7656, t7658, t7659, t7660, t7662, t7663, t7665, t7666, t7667, t7669, t7670)
}

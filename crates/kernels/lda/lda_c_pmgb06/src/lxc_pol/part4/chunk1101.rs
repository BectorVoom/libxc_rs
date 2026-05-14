//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1101/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1101<F: Float>(t432: F, t6626: F, t1604: F, t2563: F, t1600: F, t2553: F, t1602: F, t161: F, t166: F, t132: F, t137: F, t2106: F, t5039: F, t9619: F, t153: F, t1864: F, t439: F, t4779: F) -> (F, F, F, F, F, F) {
    let t16455 = t432 * t6626;
    let t16456 = 4.0 / 45.0 * t16455;
    let t16458 = t2563 * t1604 / 15.0;
    let t16459 = t2553 * t1600;
    let t16463 = t161 * t166 * t16459 * t1602 / 15.0;
    let t16467 = t132 * t137 * t2106 * t5039 / 15.0;
    let t16468 = 2.0 / 135.0 * t9619;
    let t16472 = 8.0 / 45.0 * t439 * t4779 * t153 * t1864;
    (t16456, t16458, t16463, t16467, t16468, t16472)
}

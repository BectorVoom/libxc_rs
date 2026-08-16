//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 824/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk824<F: Float>(t7778: F, t7800: F, t465: F, t137: F, t132: F, t2599: F, t851: F, t3458: F, t166: F, t161: F, t2604: F, t822: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7801 = t7778 + t7800;
    let t7802 = t465 * t7801;
    let t7803 = t137 * t7802;
    let t7805 = t132 * t7803 / F::cast_from(30.0_f64);
    let t7806 = t2599 * t851;
    let t7807 = t3458 * t7806;
    let t7808 = t166 * t7807;
    let t7810 = t161 * t7808 / F::cast_from(5.0_f64);
    let t7811 = t2604 * t822;
    (t7801, t7802, t7803, t7805, t7806, t7807, t7808, t7810, t7811)
}

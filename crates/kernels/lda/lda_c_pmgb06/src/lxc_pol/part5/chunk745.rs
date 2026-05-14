//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 745/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk745<F: Float>(t497: F, t7300: F, t506: F, t36: F, t2900: F, t4911: F, t6800: F, t6811: F, t6819: F, t7596: F, t7600: F, t7603: F, t7607: F, t7610: F, t176: F, t166: F) -> (F, F, F, F, F, F) {
    let t7612 = t497 * t7300;
    let t7613 = t506 * t7612;
    let t7614 = t36 * t7613;
    let t7616 = t2900 + 0.002518888888888889 * t4911 - 0.0012594444444444445 * t6800 + 0.003778333333333333 * t6811 - 0.0018891666666666666 * t6819 + 0.002099074074074074 * t7596 - 0.007556666666666666 * t7600 + 0.003778333333333333 * t7603 + 0.011335 * t7607 - 0.011335 * t7610 + 0.0018891666666666666 * t7614;
    let t7617 = t7616 * t176;
    let t7618 = t166 * t7617;
    (t7612, t7613, t7614, t7616, t7617, t7618)
}

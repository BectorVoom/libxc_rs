//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 886/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk886<F: Float>(t3443: F, t802: F, t9330: F, t9332: F, t1988: F, t3203: F, t493: F, t11842: F, t11843: F, t9338: F, t9340: F, t9342: F, t9345: F, t9348: F, t1992: F, t3459: F, t851: F, t9636: F) -> (F, F, F, F, F, F) {
    let t11845 = t802 * t3443 / 30.0;
    let t11846 = 4.0 / 135.0 * t9330;
    let t11847 = 2.0 / 45.0 * t9332;
    let t11853 = 2.0 / 15.0 * t493 * t1988 * t3203;
    let t11854 = -t11842 + t11843 - t11845 + t11846 - t11847 + 0.09973633333333333 * t9338 + 0.299209 * t9340 - 0.19947266666666666 * t9342 - t9345 + t9348 + t11853;
    let t11859 = 4.0 / 5.0 * t493 * t1992 * t9636 * t851 * t3459;
    (t11845, t11846, t11847, t11853, t11854, t11859)
}

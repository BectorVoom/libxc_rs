//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 776/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk776<F: Float>(t348: F, t350: F, t3611: F, t3500: F, t3510: F, t61: F, t8337: F, t3616: F, t365: F, t1179: F, t342: F, t55: F, t1276: F, t1243: F, t1180: F, t361: F) -> (F, F, F, F, F, F) {
    let t8341 = t348 * t3611 * t350;
    let t8346 = 0.16322666666666666 * t61 * t3500 * t3510 * t8337;
    let t8348 = t365 * t3616 * t350;
    let t8352 = t55 * t1179 * t342;
    let t8353 = t1276 * t8352;
    let t8355 = t1243 * t8352;
    let t8357 = t1180 * t361;
    (t8341, t8346, t8348, t8353, t8355, t8357)
}

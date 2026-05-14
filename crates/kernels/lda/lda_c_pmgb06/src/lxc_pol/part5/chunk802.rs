//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 802/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk802<F: Float>(t1437: F, t1830: F, t455: F, t1530: F, t1710: F, t485: F, t1687: F, t1730: F, t4159: F, t573: F, t580: F, t206: F, t208: F, t247: F, t161: F, t3004: F, t512: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9189 = t1437 * t1437;
    let t9190 = 1.0 / t9189;
    let t9215 = t1830 * t455;
    let t9220 = 1.0 / t1437 / t1530;
    let t9266 = t485 * t1710;
    let t9340 = t1687 * t1730;
    let t9342 = t573 * t4159;
    let t9345 = 0.26596355555555556 * t580 * t4159;
    let t9348 = 0.19208479012345678 * t206 * t247 * t208;
    let t9350 = t161 * t3004 * t512;
    (t9190, t9215, t9220, t9266, t9340, t9342, t9345, t9348, t9350)
}

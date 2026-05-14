//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 717/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk717<F: Float>(t3004: F, t9291: F, t1404: F, t2982: F, t3084: F, t5462: F, t612: F, t1671: F, t5549: F, t3638: F, t458: F, t1653: F, t122: F, t1803: F, t2995: F, t1303: F, t134: F) -> (F, F, F, F, F, F, F, F) {
    let t9292 = t3004 * t9291;
    let t9294 = t2982 * t1404;
    let t9295 = t3084 * t9294;
    let t9297 = t5462 * t612;
    let t9298 = t1671 * t5549;
    let t9299 = t9297 * t9298;
    let t9301 = t3638 * t458;
    let t9302 = t1653 * t9301;
    let t9304 = t1803 * t122;
    let t9305 = t9304 * t2995;
    let t9306 = t134 * t1303;
    (t9292, t9294, t9295, t9299, t9302, t9304, t9305, t9306)
}

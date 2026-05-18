//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1227/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1227<F: Float>(t11450: F, t8193: F, t11454: F, t2606: F, t857: F, t3917: F, t3918: F, t3884: F, t3886: F, t2723: F, t7257: F, t10959: F, t2812: F, t8164: F) -> (F, F, F, F, F, F) {
    let t25335 = t11450 * t8193;
    let t25338 = t11454 * t8193;
    let t25341 = t857 * t2606;
    let t25343 = t3917 * t25341 * t3918;
    let t25346 = t3884 * t25341 * t3886;
    let t25348 = t2723 * t7257;
    let t25353 = t2812 * t10959 * t8164;
    (t25335, t25338, t25343, t25346, t25348, t25353)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 748/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk748<F: Float>(t1014: F, t287: F, t7205: F, t1010: F, t2253: F, t2555: F, t23: F, t864: F, t191: F, t2437: F, t2433: F, t2331: F, t8: F) -> (F, F, F, F, F, F, F, F) {
    let t7207 = t7205 * t287 * t1014;
    let t7208 = t1010 * t7207;
    let t7210 = t2555 * t2253;
    let t7212 = t23 * t864;
    let t7213 = t7212 * t191;
    let t7214 = t7213 * t2437;
    let t7215 = t2433 * t7214;
    let t7217 = t2331 * t8;
    (t7207, t7208, t7210, t7212, t7213, t7214, t7215, t7217)
}

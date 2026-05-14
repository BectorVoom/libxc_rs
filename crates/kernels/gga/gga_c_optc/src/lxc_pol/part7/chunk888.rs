//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 888/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk888<F: Float>(t3245: F, t9058: F, t3105: F, t3117: F, t3120: F, t3235: F, t2850: F, t3236: F, t4387: F, t1: F, t3107: F, t1028: F, t8483: F, t914: F, t8533: F, t2367: F, t3097: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9059 = t3245 * t9058;
    let t9062 = t3117 * t3105;
    let t9063 = t9062 * t3120;
    let t9066 = t3235 * t9058;
    let t9069 = t2850 * t3236;
    let t9070 = t4387 * t9069;
    let t9073 = t3107 * t1;
    let t9074 = t9073 * t1028;
    let t9075 = t9062 * t9074;
    let t9078 = t914 * t8483;
    let t9081 = t914 * t8533;
    let t9084 = t2367 * t3097;
    (t9059, t9062, t9063, t9066, t9069, t9070, t9073, t9074, t9075, t9078, t9081, t9084)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1140/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1140<F: Float>(t25035: F, t953: F, t140: F, t2246: F, t2665: F, t2748: F, t2661: F, t2708: F, t8240: F, t2746: F, t301: F, t327: F, t24565: F, t25749: F, t3907: F, t3909: F) -> (F, F, F, F, F, F) {
    let t25920 = t953 * t25035;
    let t25928 = t2246 * t2665 * t140;
    let t25929 = t2748 * t25928;
    let t25932 = t2661 * t25928;
    let t25935 = t2708 * t8240;
    let t25939 = 1.0 / t2746 / t327 * t301;
    let t25940 = t25939 * t24565;
    let t25946 = t3907 * t25749 * t3909;
    (t25920, t25929, t25932, t25935, t25940, t25946)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1124/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1124<F: Float>(t25052: F, t953: F, t2765: F, t7878: F, t940: F, t2708: F, t8257: F, t7898: F, t871: F, t938: F, t2367: F, t8062: F, t913: F, t2670: F, t7481: F, t1: F, t1885: F, t24468: F) -> (F, F, F, F, F, F, F) {
    let t25524 = t953 * t25052;
    let t25529 = t940 * t7878 * t2765;
    let t25531 = t2708 * t8257;
    let t25534 = t938 * t7898 * t871;
    let t25538 = t913 * t2367 * t8062;
    let t25540 = t7481 * t2670;
    let t25542 = t24468 * t1885 * t1;
    (t25524, t25529, t25531, t25534, t25538, t25540, t25542)
}

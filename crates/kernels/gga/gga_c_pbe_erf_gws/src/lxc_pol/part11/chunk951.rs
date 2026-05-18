//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 951/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk951<F: Float>(t1022: F, t7758: F, t1033: F, t4908: F, t1023: F, t2735: F, t616: F, t1018: F, t185: F, t1: F, t5560: F, t8465: F) -> (F, F, F, F, F) {
    let t22917 = t7758 * t1022;
    let t22934 = t1033 * t4908;
    let t22939 = t616 * t2735 * t1023;
    let t22967 = t2735 * t1018;
    let t22968 = t185 * t22967;
    let t22982 = t8465 * t1 * t5560;
    (t22917, t22934, t22939, t22968, t22982)
}

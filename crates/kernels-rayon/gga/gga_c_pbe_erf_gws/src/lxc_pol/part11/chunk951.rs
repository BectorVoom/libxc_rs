//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 951/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk951(t1022: f64, t7758: f64, t1033: f64, t4908: f64, t1023: f64, t2735: f64, t616: f64, t1018: f64, t185: f64, t1: f64, t5560: f64, t8465: f64) -> (f64, f64, f64, f64, f64) {
    let t22917 = t7758 * t1022;
    let t22934 = t1033 * t4908;
    let t22939 = t616 * t2735 * t1023;
    let t22967 = t2735 * t1018;
    let t22968 = t185 * t22967;
    let t22982 = t8465 * t1 * t5560;
    (t22917, t22934, t22939, t22968, t22982)
}

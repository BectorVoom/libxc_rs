//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1052/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1052(t11807: f64, t2147: f64, t2146: f64, t2164: f64, t3832: f64, t11363: f64, t6659: f64, t858: f64, t884: f64, t2142: f64, t3783: f64, t1134: f64, t3189: f64) -> (f64, f64, f64, f64, f64) {
    let t11808 = t2147 * t11807;
    let t11810 = t2146 * t11808 / 48.0_f64;
    let t11811 = t2164 * t3832;
    let t11812 = 7.0_f64 / 288.0_f64 * t11811;
    let t11814 = t6659 * t858 * t11363;
    let t11816 = t884 * t11814 / 4.0_f64;
    let t11817 = t3783 * t2142;
    let t11818 = 7.0_f64 / 288.0_f64 * t11817;
    let t11819 = t1134 * t3189;
    (t11810, t11812, t11816, t11818, t11819)
}

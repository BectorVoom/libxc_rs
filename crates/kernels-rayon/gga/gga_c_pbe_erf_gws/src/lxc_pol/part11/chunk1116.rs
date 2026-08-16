//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1116/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1116(t41056: f64, t41061: f64, t41069: f64, t41074: f64, t12550: f64, t2615: f64, t47400: f64, t587: f64, t590: f64, t591: f64, t10848: f64, t3531: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47809 = 32.0_f64 / 135.0_f64 * t41056;
    let t47810 = 64.0_f64 / 15.0_f64 * t41061;
    let t47811 = 64.0_f64 / 15.0_f64 * t41069;
    let t47812 = 32.0_f64 / 15.0_f64 * t41074;
    let t47814 = 16.0_f64 / 45.0_f64 * t2615 * t12550;
    let t47818 = 4.0_f64 / 45.0_f64 * t587 * t590 * t591 * t47400;
    let t47820 = 8.0_f64 / 9.0_f64 * t10848 * t3531;
    (t47809, t47810, t47811, t47812, t47814, t47818, t47820)
}

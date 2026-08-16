//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1158/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1158(t20693: f64, t20698: f64, t822: f64, t6253: f64, t6563: f64, t2100: f64, t816: f64, t2074: f64, t2157: f64, t2170: f64, t3138: f64, t6177: f64) -> (f64, f64, f64, f64, f64) {
    let t20700 = t822 * t20693 * t20698 / 16.0_f64;
    let t20702 = 3.0_f64 / 8.0_f64 * t6253 * t6563;
    let t20703 = t816 * t2100;
    let t20708 = t2157 * t2074;
    let t20712 = t3138 * t2170 * t6177 * t20708 / 4.0_f64;
    (t20700, t20702, t20703, t20708, t20712)
}

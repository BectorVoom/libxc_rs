//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta45 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk329;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk330;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk331;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk332;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk333;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk334;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk335;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta45(t606: f64, t905: f64, t904: f64, t128: f64, t903: f64, t291: f64, t287: f64, t275: f64, t276: f64, t902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t906 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk329(t606, t905);
        let (t907, t908) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk330(t904, t906, t128);
        let t910 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk331(t903, t908);
        let (t912, t913, t914) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk332(t291, t910, t287);
        let t915 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk333(t275, t914);
        let t916 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk334(t276);
        let t918 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk335(t902, t908);
    (t906, t907, t908, t910, t912, t913, t914, t915, t916, t918)
}

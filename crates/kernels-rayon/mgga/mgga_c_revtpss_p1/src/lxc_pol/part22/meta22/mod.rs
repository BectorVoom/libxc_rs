//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta22 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk170;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk171;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk172;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk173;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk174;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta22(t408: f64, t422: f64, t406: f64, t409: f64, t412: f64, t416: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t424, t426) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk170(t408, t422, t406);
        let (t431, t434, t435) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk171(t406, t409, t412, t416);
        let t439 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk172(t406);
        let (t444, t447, t448) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk173(t406, t409, t412, t416);
        let (t452, t454, t456) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk174(t439, t448, t300, t424, t426, t435, t406);
    (t424, t426, t431, t434, t435, t439, t444, t447, t448, t452, t454, t456)
}

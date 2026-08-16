//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta31 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk233;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk234;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk235;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk236;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk237;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta31(t57: f64, t81: f64, t606: f64, t633: f64, t77: f64, t608: f64, t628: f64, t71: f64, t85: f64, t5: f64, t599: f64, t603: f64, t91: f64, t117: f64, t116: f64, t94: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t635 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk233(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk234(t635, t81);
        let (t640, t641) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk235(t606, t633, t637, t77);
        let t644 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk236(t608, t628, t641, t71, t85);
        let (t648, t649) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk237(t5, t599, t603, t644, t91, t117);
        let t651 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk238(t116, t94);
    (t635, t637, t640, t641, t644, t648, t649, t651)
}

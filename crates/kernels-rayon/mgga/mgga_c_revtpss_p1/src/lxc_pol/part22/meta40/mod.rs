//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk294;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk295;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk296;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk297;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk298;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk299;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta40(t159: f64, t794: f64, t222: f64, t228: f64, t216: f64, t136: f64, t220: f64, t124: f64, t775: f64, t212: f64, t27: f64, t235: f64, t240: f64, t234: f64, t243: f64, t236: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t795, t797, t798, t799) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk294(t159, t794, t222, t228, t216);
        let t800 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk295(t136, t220);
        let (t802, t807) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk296(t124, t775, t800, t212, t27);
        let t808 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk297(t235, t240);
        let t810 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk298(t234, t243, t808);
        let (t812, t813, t814) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk299(t807, t810, t236, t786, t240, t27);
    (t795, t797, t798, t799, t800, t802, t807, t808, t810, t812, t813, t814)
}

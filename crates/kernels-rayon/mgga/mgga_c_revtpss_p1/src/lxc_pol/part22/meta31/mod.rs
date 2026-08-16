//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta31 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk231;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk232;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk233;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk234;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk235;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk236;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta31(t36: f64, t606: f64, t70: f64, t39: f64, t41: f64, rho0: f64, sigma0: f64, t48: f64, t60: f64, t579: f64, t66: f64, t64: f64, t44: f64, t49: f64, t56: f64, t38: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t607 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk231(t36, t606);
        let t608 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk232(t607, t70);
        let (t611, t613, t614) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk233(t39, t41, rho0, sigma0);
        let (t617, t620, t624) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk234(t48, t606, t60, t579, t66);
        let t625 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk235(t624, t64);
        let (t626, t627) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk236(t625, t44, t49, t56, t614, t617, t620);
        let t628 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk237(t38, t627);
    (t607, t608, t611, t613, t614, t617, t620, t624, t625, t626, t627, t628)
}

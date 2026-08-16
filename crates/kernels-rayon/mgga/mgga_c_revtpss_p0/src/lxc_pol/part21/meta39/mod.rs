//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta39 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk291;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk292;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk293;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk294;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk295;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk296;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta39(t252: f64, t786: f64, t257: f64, t72: f64, t686: f64, t579: f64, t65: f64, t64: f64, t159: f64, t222: f64, t228: f64, t216: f64, t136: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t787, t788) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk291(t252, t786, t257, t72);
        let t789 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk292(t686, t788);
        let (t791, t793) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk293(t787, t789, t579, t65);
        let t794 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk294(t64, t793);
        let (t795, t797, t798, t799) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk295(t159, t794, t222, t228, t216);
        let t800 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk296(t136, t220);
    (t787, t788, t789, t791, t793, t794, t795, t797, t798, t799, t800)
}

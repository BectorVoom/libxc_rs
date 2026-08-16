//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta6 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk48;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk49;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk50;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk51;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk52;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk53;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta6(t30: f64, t53: f64, rho1: f64, tau1: f64, t33: f64, t97: f64, t69: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t98, t99, t100) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk48(t30);
        let (t101, t105) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk49(t100, t98, t53, rho1, tau1);
        let (t106, t107, t108) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk50(t33);
        let t109 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk51(t106, t108);
        let (t111, t112) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk52(t101, t105, t109, t97);
        let (t116, t114) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk53(t112, t69);
    (t98, t99, t100, t105, t106, t107, t108, t109, t111, t112, t116, t114)
}

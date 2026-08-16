//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta7 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk47;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk48;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk49;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk50;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk51;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk52;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk53;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk54;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta7(t106: f64, t108: f64, t101: f64, t105: f64, t97: f64, t69: f64, t94: f64, t30: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109, t111, t112) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk47(t106, t108, t101, t105, t97);
        let (t116, t114) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk48(t112, t69);
        let t117 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk49(t116);
        let t118 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk50(t117, t94);
        let (t121, t122) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk51(t30, dens_threshold, rho0, zeta_threshold);
        let t123 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk52(t122, t72);
        let t124 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk53();
        let t125 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk54(t124);
    (t109, t111, t112, t116, t114, t117, t118, t121, t122, t123, t124, t125)
}

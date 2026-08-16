//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta7 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk48;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk49;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk50;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk51;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk52;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk53;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk54;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk55;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta7<F: Float>(t106: F, t108: F, t101: F, t105: F, t97: F, t69: F, t94: F, t30: F, dens_threshold: F, rho0: F, zeta_threshold: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t109, t111, t112) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk48::<F>(t106, t108, t101, t105, t97);
        let (t116, t114) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk49::<F>(t112, t69);
        let t117 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk50::<F>(t116);
        let t118 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk51::<F>(t117, t94);
        let (t121, t122) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk52::<F>(t30, dens_threshold, rho0, zeta_threshold);
        let t123 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk53::<F>(t122, t72);
        let t124 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk54::<F>();
        let t125 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk55::<F>(t124);
    (t109, t111, t112, t116, t114, t117, t118, t121, t122, t123, t124, t125)
}

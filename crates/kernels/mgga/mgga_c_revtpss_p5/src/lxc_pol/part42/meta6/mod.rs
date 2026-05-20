//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta6 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk39;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk40;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk41;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk42;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk43;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk44;
use chunk6::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk45;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta6<F: Float>(t93: F, t41: F, rho0: F, tau0: F, t30: F, t53: F, rho1: F, tau1: F, t33: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t94 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk39::<F>(t93);
        let t97 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk40::<F>(t41, rho0, tau0);
        let (t98, t99, t100) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk41::<F>(t30);
        let (t101, t104) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk42::<F>(t100, t98, t53, rho1);
        let t105 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk43::<F>(t104, tau1);
        let (t106, t107, t108) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk44::<F>(t33);
        let t109 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk45::<F>(t106, t108);
    (t94, t97, t98, t99, t100, t101, t104, t105, t106, t107, t108, t109)
}

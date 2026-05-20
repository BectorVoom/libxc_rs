//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta14 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk96;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk97;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk98;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk99;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk100;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk101;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta14<F: Float>(t206: F, t241: F, t137: F, t72: F, t125: F, t217: F, t222: F, t237: F, t225: F, t234: F, t213: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t242, t243) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk96::<F>(t206);
        let (t244, t245) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk97::<F>(t241, t243, t137);
        let (t246, t247) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk98::<F>(t245, t72, t125);
        let t251 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk99::<F>(t244, t247, t217, t222, t237);
        let t252 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk100::<F>(t225, t251);
        let (t253, t256, t257) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk101::<F>(t234, t251, t213);
    (t242, t243, t245, t246, t247, t251, t252, t253, t256, t257)
}

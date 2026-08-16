//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta33 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk209;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk210;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk211;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk212;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk213;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk214;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta33<F: Float>(t608: F, t628: F, t641: F, t71: F, t85: F, t5: F, t599: F, t603: F, t91: F, t117: F, t116: F, t94: F, t112: F, t625: F, t111: F, t43: F, t605: F, tau0: F) -> (F, F, F, F, F, F, F, F, F) {
        let t644 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk209::<F>(t608, t628, t641, t71, t85);
        let t648 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk210::<F>(t5, t599, t603, t644, t91);
        let t649 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk211::<F>(t117, t648);
        let t651 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk212::<F>(t116, t94);
        let (t653, t654, t655) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk213::<F>(t112, t625, t111);
        let (t656, t658) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk214::<F>(t43, t605, tau0);
    (t644, t648, t649, t651, t653, t654, t655, t656, t658)
}

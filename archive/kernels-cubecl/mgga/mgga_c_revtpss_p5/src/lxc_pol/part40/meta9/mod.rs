//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta9 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk62;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk63;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk64;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk65;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk66;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk67;
use chunk6::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk68;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta9<F: Float>(t128: F, t131: F, t134: F, t141: F, t130: F, t37: F, t45: F, zeta_threshold: F, t79: F, t57: F, t82: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t143, t146, t147, t149) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk62::<F>(t128, t131, t134, t141, t130);
        let t150 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk63::<F>(t37);
        let (t152, t153) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk64::<F>(t45, zeta_threshold);
        let t157 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk65::<F>(t45, t153, t79, t57, t82, zeta_threshold);
        let (t158, t159) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk66::<F>(t150, t157);
        let t162 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk67::<F>(t159);
        let t164 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk68::<F>(t128);
    (t143, t146, t147, t149, t150, t152, t153, t157, t158, t159, t162, t164)
}

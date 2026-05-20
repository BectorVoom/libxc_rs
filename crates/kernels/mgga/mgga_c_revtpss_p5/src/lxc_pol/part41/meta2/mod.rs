//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta2 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk16;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk17;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk18;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk19;
use chunk4::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk20;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta2<F: Float>(t37: F, rho0: F, sigma0: F, t36: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t38 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk16::<F>(t37);
        let (t39, t40, t41, t43) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk17::<F>(rho0);
        let t44 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk18::<F>(t43, sigma0);
        let t45 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk19::<F>(t36);
        let (t46, t47, t48) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk20::<F>(t45);
    (t38, t39, t40, t41, t43, t44, t45, t46, t47, t48)
}

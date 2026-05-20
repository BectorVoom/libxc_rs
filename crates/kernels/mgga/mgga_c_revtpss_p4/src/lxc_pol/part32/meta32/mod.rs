//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta32 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk210;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk211;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk212;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk213;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk214;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk215;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk216;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta32<F: Float>(t36: F, t606: F, t70: F, t39: F, t41: F, rho0: F, sigma0: F, t48: F, t60: F, t579: F, t66: F, t64: F, t44: F, t49: F, t56: F, t38: F, t45: F, t78: F, t57: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t607 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk210::<F>(t36, t606);
        let (t608, t614) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk211::<F>(t607, t70, t39, t41, rho0, sigma0);
        let (t617, t620, t624) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk212::<F>(t48, t606, t60, t579, t66);
        let t625 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk213::<F>(t624, t64);
        let (t626, t627, t628, t631) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk214::<F>(t625, t44, t49, t56, t614, t617, t620, t38, t45);
        let t633 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk215::<F>(t631, t78);
        let t635 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk216::<F>(t57);
    (t607, t608, t614, t620, t624, t625, t626, t627, t628, t631, t633, t635)
}

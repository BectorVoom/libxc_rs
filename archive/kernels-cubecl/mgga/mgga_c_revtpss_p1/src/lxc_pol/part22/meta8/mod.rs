//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta8 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk59;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk60;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk61;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk62;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk63;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk64;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk65;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk66;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk67;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk68;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta8<F: Float>(t65: F, t125: F, t123: F, t72: F, t122: F, t66: F, t124: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t126 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk59::<F>(t65);
        let t127 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk60::<F>(t125, t126);
        let t128 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk61::<F>(t123, t127);
        let t130 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk62::<F>(t128);
        let t131 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk63::<F>(t128);
        let (t134, t136) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk64::<F>(t128, t72);
        let t137 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk65::<F>(t122);
        let t138 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk66::<F>(t136, t137);
        let (t139, t140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk67::<F>(t66, t124);
        let t141 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk68::<F>(t138, t140);
    (t126, t127, t128, t130, t131, t134, t136, t137, t138, t139, t140, t141)
}

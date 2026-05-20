//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta7 (260520-c91 hierarchical CSE).
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
mod chunk10;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk54;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk55;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk56;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk57;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk58;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk59;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk60;
use chunk7::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk61;
use chunk8::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk62;
use chunk9::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk63;
use chunk10::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk64;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta7<F: Float>(t116: F, t94: F, t30: F, dens_threshold: F, rho0: F, zeta_threshold: F, t72: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t117 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk54::<F>(t116);
        let t118 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk55::<F>(t117, t94);
        let t121 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk56::<F>(t30, dens_threshold, rho0, zeta_threshold);
        let (t122, t123) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk57::<F>(t121, t72);
        let t124 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk58::<F>();
        let t125 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk59::<F>(t124);
        let t126 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk60::<F>(t65);
        let t127 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk61::<F>(t125, t126);
        let t128 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk62::<F>(t123, t127);
        let t130 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk63::<F>(t128);
        let t131 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk64::<F>(t128);
    (t117, t118, t121, t122, t123, t124, t125, t126, t127, t128, t130, t131)
}

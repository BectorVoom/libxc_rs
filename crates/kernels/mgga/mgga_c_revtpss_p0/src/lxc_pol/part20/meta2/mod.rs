//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta2 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk20;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk21;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk22;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk23;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk24;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk25;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk26;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta2<F: Float>(t37: F, rho0: F, sigma0: F, t36: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t38 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk20::<F>(t37);
        let (t39, t40) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk21::<F>(rho0);
        let t41 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk22::<F>(t40);
        let (t43, t44) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk23::<F>(t39, t41, sigma0);
        let t45 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk24::<F>(t36);
        let (t46, t47, t48) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk25::<F>(t45);
        let t49 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk26::<F>(t46, t48);
    (t38, t39, t40, t41, t43, t44, t45, t46, t47, t48, t49)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk88;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk89;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk90;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk91;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk92;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk93;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk94;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk95;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta11<F: Float>(t45: F, t57: F, t78: F, t199: F, t81: F, zeta_threshold: F, t128: F, t16: F, t65: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t200, t202, t205) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk88::<F>(t45, t57, t78, t199, t81, zeta_threshold);
        let t206 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk89::<F>(t205);
        let t207 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk90::<F>(t205, t206);
        let t209 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk91::<F>(t128);
        let (t211, t212) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk92::<F>(t128);
        let t213 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk93::<F>(t209, t212);
        let t215 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk94::<F>(t16, t65);
        let t216 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk95::<F>(t215, t64);
    (t200, t202, t205, t206, t207, t209, t211, t212, t213, t215, t216)
}

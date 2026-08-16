//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta11 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk87;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk88;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk89;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk90;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk91;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk92;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk93;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk94;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta11<F: Float>(t187: F, t192: F, t73: F, t152: F, t45: F, t57: F, t78: F, t81: F, zeta_threshold: F, t128: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t194, t196) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk87::<F>(t187, t192);
        let t198 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk88::<F>(t73, t196);
        let t199 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk89::<F>(t152);
        let (t200, t202, t205) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk90::<F>(t45, t57, t78, t199, t81, zeta_threshold);
        let t206 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk91::<F>(t205);
        let t207 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk92::<F>(t205, t206);
        let t209 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk93::<F>(t128);
        let (t211, t212) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk94::<F>(t128);
    (t194, t196, t198, t199, t200, t202, t205, t206, t207, t209, t211, t212)
}

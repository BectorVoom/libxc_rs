//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk418;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk419;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk420;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk421;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk422;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk423;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk424;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta56<F: Float>(t1121: F, t606: F, t1120: F, t128: F, t1119: F, t422: F, t418: F, t408: F, t409: F, t1118: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t1122 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk418::<F>(t1121, t606);
        let (t1123, t1124) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk419::<F>(t1120, t1122, t128);
        let t1126 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk420::<F>(t1119, t1124);
        let (t1128, t1129, t1130) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk421::<F>(t1126, t422, t418);
        let t1131 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk422::<F>(t1130, t408);
        let t1132 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk423::<F>(t409);
        let t1134 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk424::<F>(t1118, t1124);
    (t1122, t1123, t1124, t1126, t1128, t1129, t1130, t1131, t1132, t1134)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta59 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk436;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk437;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk438;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk439;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk440;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk441;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta59<F: Float>(t1118: F, t1124: F, t448: F, t444: F, t439: F, t1143: F, t1135: F, t1140: F, t1147: F, t447: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1173, t1175) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk436::<F>(t1118, t1124);
        let (t1176, t1178, t1179) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk437::<F>(t1175, t448, t444);
        let t1180 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk438::<F>(t1179, t439);
        let (t1182, t1185, t1187) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk439::<F>(t1118, t1143, t1124, t1135, t1140, t1147);
        let t1188 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk440::<F>(t447);
        let t1189 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk441::<F>(t1187, t1188);
    (t1173, t1175, t1176, t1178, t1179, t1180, t1182, t1185, t1187, t1188, t1189)
}

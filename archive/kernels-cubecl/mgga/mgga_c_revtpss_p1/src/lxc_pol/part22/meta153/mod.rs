//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta153 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1017;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1018;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1019;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1020;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1021;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1022;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1023;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1024;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1025;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1026;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta153<F: Float>(t3617: F, t66: F, t3363: F, t247: F, t474: F, t479: F, t3089: F, t1285: F, t1264: F, t828: F, t1248: F, t73: F, t1121: F, t471: F, t606: F, t126: F, t1263: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3618, t3620) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1017::<F>(t3617, t66, t3363, t247);
        let t3623 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1018::<F>(t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1019::<F>(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1020::<F>(t1285, t3624);
        let t3626 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1021::<F>(t1264, t828);
        let (t3627, t3628) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1022::<F>(t1248, t73, t1121, t471);
        let t3629 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1023::<F>(t3628, t606);
        let t3630 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1024::<F>(t3627, t3629);
        let t3631 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1025::<F>(t3626, t3630);
        let t3634 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1026::<F>(t126, t1263);
    (t3618, t3620, t3623, t3624, t3625, t3626, t3627, t3628, t3629, t3630, t3631, t3634)
}

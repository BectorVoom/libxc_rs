//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta160 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1016;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1017;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1018;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1019;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1020;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1021;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1022;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1023;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1024;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta160<F: Float>(t225: F, t3727: F, t494: F, t1269: F, t460: F, t1275: F, t493: F, t1294: F, t1204: F, t1284: F, t1280: F, t3568: F, t487: F, t1209: F, t1287: F, t3721: F, t473: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3729, t3732) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1016::<F>(t225, t3727, t494, t1269, t460);
        let (t3736, t3737) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1017::<F>(t1275, t493, t225);
        let t3738 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1018::<F>(t1294);
        let t3739 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1019::<F>(t3737, t3738);
        let t3746 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1020::<F>(t1204, t1284);
        let (t3751, t3754) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1021::<F>(t1280, t3568, t1284, t487);
        let t3755 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1022::<F>(t1209, t3754);
        let t3756 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1023::<F>(t1287, t3721);
        let t3759 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1024::<F>(t1269, t473);
    (t3729, t3732, t3736, t3737, t3738, t3739, t3746, t3751, t3754, t3755, t3756, t3759)
}

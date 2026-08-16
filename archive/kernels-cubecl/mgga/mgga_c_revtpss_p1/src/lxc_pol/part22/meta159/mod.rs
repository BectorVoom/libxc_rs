//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta159 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1058;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1059;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1060;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1061;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1062;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1063;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1064;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta159<F: Float>(t1204: F, t1284: F, t1280: F, t3568: F, t487: F, t1209: F, t1287: F, t3721: F, t1269: F, t473: F, t1214: F, t3584: F, t3140: F, t3596: F, t460: F, t3601: F, t3303: F, t3603: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3746 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1058::<F>(t1204, t1284);
        let (t3751, t3754) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1059::<F>(t1280, t3568, t1284, t487);
        let t3755 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1060::<F>(t1209, t3754);
        let t3756 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1061::<F>(t1287, t3721);
        let t3759 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1062::<F>(t1269, t473);
        let (t3760, t3763, t3766) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1063::<F>(t1214, t3759, t1280, t3584, t3140, t3596);
        let t3767 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1064::<F>(t3766, t460);
        let (t3768, t3769) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1065::<F>(t3601, t487, t3303, t3603);
    (t3746, t3751, t3754, t3755, t3756, t3759, t3760, t3763, t3766, t3767, t3768, t3769)
}

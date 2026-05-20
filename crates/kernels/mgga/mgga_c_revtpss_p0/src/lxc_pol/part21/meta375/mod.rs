//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta375 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1780;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1781;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1782;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1783;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1784;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1785;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta375<F: Float>(t3373: F, t689: F, t159: F, t3617: F, t12257: F, t128: F, t12269: F, t3360: F, t1120: F, t12273: F, t12287: F, t12277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t12303 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1780::<F>(t3373, t689);
        let (t12305, t12306, t12307) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1781::<F>(t159, t3617, t12257, t128);
        let (t12309, t12310) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1782::<F>(t12269, t3360, t128);
        let (t12313, t12314) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1783::<F>(t1120, t12273, t128);
        let (t12316, t12317) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1784::<F>(t1120, t12287, t128);
        let (t12319, t12320) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1785::<F>(t1120, t12277, t128);
    (t12303, t12305, t12306, t12307, t12309, t12310, t12313, t12314, t12316, t12317, t12319, t12320)
}

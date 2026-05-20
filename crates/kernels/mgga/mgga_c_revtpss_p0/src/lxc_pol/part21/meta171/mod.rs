//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta171 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1075;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1076;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1077;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1078;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1079;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1080;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta171<F: Float>(t1398: F, t543: F, t550: F, t3992: F, t2661: F, t1384: F, t544: F, t235: F, t239: F, t820: F, t3923: F, t1390: F, t828: F, t531: F, t549: F, t240: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3994, t3995, t3996, t3999) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1075::<F>(t1398, t543, t550, t3992, t2661, t1384, t544);
        let t4000 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1076::<F>(t235, t3999);
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1077::<F>(t239, t4000, t820, t543);
        let t4004 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1078::<F>(t3923, t4003);
        let (t4006, t4010) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1079::<F>(t1390, t4004, t828, t531, t549);
        let (t4011, t4012) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1080::<F>(t240, t4010, t72);
    (t3994, t3995, t3996, t3999, t4000, t4002, t4003, t4004, t4006, t4010, t4011, t4012)
}

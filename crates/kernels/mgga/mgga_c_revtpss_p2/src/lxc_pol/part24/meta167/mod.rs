//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta167 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk827;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk828;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk829;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk830;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta167<F: Float>(t480: F, t6601: F, t482: F, t6573: F, t371: F, t372: F, t1715: F, t5277: F, t1042: F, t6435: F, t6437: F, t6441: F, t6473: F, t6476: F, t6542: F, t6544: F, t6546: F, t6550: F, t6554: F, t6558: F, t1250: F, t1794: F, t3604: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6602, t6609, t6611) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk827::<F>(t480, t6601, t482, t6573, t371, t372);
        let (t6618, t6619, t6622) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk828::<F>(t1715, t5277, t1042, t6435, t6437, t6441, t6473, t6476, t6542, t6544, t6546, t6550, t6554, t6558);
        let (t6624, t6625) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk829::<F>(t1250, t482, t6622, t1042);
        let t6628 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk830::<F>(t1794);
        let (t6629, t6630, t6631) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk831::<F>(t482, t6628, t3604, t1042);
    (t6602, t6609, t6611, t6618, t6619, t6622, t6624, t6625, t6628, t6629, t6630, t6631)
}

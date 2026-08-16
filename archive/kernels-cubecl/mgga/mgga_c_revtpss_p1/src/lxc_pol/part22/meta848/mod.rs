//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2987;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta848<F: Float>(t5600: F, t9292: F, t1893: F, t4075: F, t786: F, t9682: F, t10115: F, t1894: F, t14094: F, t2435: F, t1358: F, t2439: F, t5710: F, t785: F, t2782: F, t4077: F, t47794: F, t556: F, t1426: F, t5711: F, t3917: F, t3899: F, t5775: F, t689: F, t14100: F, t9686: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t49468, t49471, t49472, t49474, t49476, t49480) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2987::<F>(t5600, t9292, t1893, t4075, t786, t9682, t10115, t1894, t14094, t2435, t1358, t2439, t5710, t785);
        let (t49497, t49503, t49504, t49508, t49512) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2988::<F>(t2782, t4077, t47794, t556, t1426, t5711, t786, t3917, t3899, t5775, t689, t14100, t9686);
    (t49468, t49471, t49472, t49474, t49476, t49480, t49497, t49503, t49504, t49508, t49512)
}

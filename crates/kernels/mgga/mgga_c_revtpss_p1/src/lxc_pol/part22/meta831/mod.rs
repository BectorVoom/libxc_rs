//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2952;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta831<F: Float>(t14055: F, t9775: F, t1885: F, t46722: F, t13867: F, t221: F, t3978: F, t9921: F, t14047: F, t14051: F, t1412: F, t5658: F, t2661: F, t3938: F, t3992: F, t14045: F, t9810: F, t13774: F, t1399: F, t13927: F, t48100: F, t9816: F, t13910: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t48516, t48518, t48527, t48529, t48531, t48533) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2952::<F>(t14055, t9775, t1885, t46722, t13867, t221, t3978, t9921, t14047, t14051, t1412, t5658);
        let (t48536, t48540, t48544, t48548, t48553) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2953::<F>(t2661, t3938, t3992, t48533, t14045, t9810, t13774, t1399, t13927, t48100, t9816, t13910);
    (t48516, t48518, t48527, t48529, t48531, t48533, t48536, t48540, t48544, t48548, t48553)
}

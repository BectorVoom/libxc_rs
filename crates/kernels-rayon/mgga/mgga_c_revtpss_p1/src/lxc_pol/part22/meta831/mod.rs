//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta831 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2952;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta831(t14055: f64, t9775: f64, t1885: f64, t46722: f64, t13867: f64, t221: f64, t3978: f64, t9921: f64, t14047: f64, t14051: f64, t1412: f64, t5658: f64, t2661: f64, t3938: f64, t3992: f64, t14045: f64, t9810: f64, t13774: f64, t1399: f64, t13927: f64, t48100: f64, t9816: f64, t13910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48516, t48518, t48527, t48529, t48531, t48533) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2952(t14055, t9775, t1885, t46722, t13867, t221, t3978, t9921, t14047, t14051, t1412, t5658);
        let (t48536, t48540, t48544, t48548, t48553) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2953(t2661, t3938, t3992, t48533, t14045, t9810, t13774, t1399, t13927, t48100, t9816, t13910);
    (t48516, t48518, t48527, t48529, t48531, t48533, t48536, t48540, t48544, t48548, t48553)
}

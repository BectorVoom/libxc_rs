//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2987;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2988;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta848(t5600: f64, t9292: f64, t1893: f64, t4075: f64, t786: f64, t9682: f64, t10115: f64, t1894: f64, t14094: f64, t2435: f64, t1358: f64, t2439: f64, t5710: f64, t785: f64, t2782: f64, t4077: f64, t47794: f64, t556: f64, t1426: f64, t5711: f64, t3917: f64, t3899: f64, t5775: f64, t689: f64, t14100: f64, t9686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49468, t49471, t49472, t49474, t49476, t49480) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2987(t5600, t9292, t1893, t4075, t786, t9682, t10115, t1894, t14094, t2435, t1358, t2439, t5710, t785);
        let (t49497, t49503, t49504, t49508, t49512) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2988(t2782, t4077, t47794, t556, t1426, t5711, t786, t3917, t3899, t5775, t689, t14100, t9686);
    (t49468, t49471, t49472, t49474, t49476, t49480, t49497, t49503, t49504, t49508, t49512)
}

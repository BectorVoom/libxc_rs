//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2959;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2960;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta810(t11670: f64, t370: f64, t16094: f64, t11922: f64, t16021: f64, t4899: f64, t3091: f64, t43240: f64, t4787: f64, t1043: f64, t43279: f64, t15785: f64, t12160: f64, t15688: f64, t1011: f64, t15689: f64, t15692: f64, t15693: f64, t15906: f64, t15907: f64, t16012: f64, t16081: f64, t16098: f64, t3117: f64, t42546: f64, t4915: f64, t4919: f64, t51869: f64, t51905: f64, t51998: f64, t53545: f64, t1087: f64, t43065: f64, t3105: f64, t4857: f64, t1012: f64, t43222: f64, t16190: f64, t3173: f64, t11714: f64, t15144: f64, t15830: f64, t16095: f64, t16096: f64, t16196: f64, t16223: f64, t3092: f64, t3101: f64, t3106: f64, t3130: f64, t4803: f64, t51851: f64, t51856: f64, t51925: f64, t51930: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t53884, t53885, t53898, t53901, t53904, t53909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2959(t11670, t370, t16094, t11922, t16021, t4899, t3091, t43240, t4787, t1043, t43279, t15785);
        let t53920 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2960(t12160, t15688, t1011, t15689, t15692, t15693, t15906, t15907, t16012, t16081, t16098, t3117, t42546, t4915, t4919, t51869, t51905, t51998, t53545, t53885, t53898, t53901, t53904, t53909);
        let (t53923, t53954) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2961(t1087, t43065, t3105, t4857, t1012, t43222, t16190, t3173, t1011, t11714, t15144, t15830, t16012, t16095, t16096, t16196, t16223, t3092, t3101, t3106, t3130, t4803, t4919, t51851, t51856, t51925, t51930);
    (t53884, t53885, t53904, t53909, t53920, t53923, t53954)
}

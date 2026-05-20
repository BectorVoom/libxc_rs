//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2959;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2960;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2961;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta810<F: Float>(t11670: F, t370: F, t16094: F, t11922: F, t16021: F, t4899: F, t3091: F, t43240: F, t4787: F, t1043: F, t43279: F, t15785: F, t12160: F, t15688: F, t1011: F, t15689: F, t15692: F, t15693: F, t15906: F, t15907: F, t16012: F, t16081: F, t16098: F, t3117: F, t42546: F, t4915: F, t4919: F, t51869: F, t51905: F, t51998: F, t53545: F, t1087: F, t43065: F, t3105: F, t4857: F, t1012: F, t43222: F, t16190: F, t3173: F, t11714: F, t15144: F, t15830: F, t16095: F, t16096: F, t16196: F, t16223: F, t3092: F, t3101: F, t3106: F, t3130: F, t4803: F, t51851: F, t51856: F, t51925: F, t51930: F) -> (F, F, F, F, F, F, F) {
        let (t53884, t53885, t53898, t53901, t53904, t53909) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2959::<F>(t11670, t370, t16094, t11922, t16021, t4899, t3091, t43240, t4787, t1043, t43279, t15785);
        let t53920 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2960::<F>(t12160, t15688, t1011, t15689, t15692, t15693, t15906, t15907, t16012, t16081, t16098, t3117, t42546, t4915, t4919, t51869, t51905, t51998, t53545, t53885, t53898, t53901, t53904, t53909);
        let (t53923, t53954) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2961::<F>(t1087, t43065, t3105, t4857, t1012, t43222, t16190, t3173, t1011, t11714, t15144, t15830, t16012, t16095, t16096, t16196, t16223, t3092, t3101, t3106, t3130, t4803, t4919, t51851, t51856, t51925, t51930);
    (t53884, t53885, t53904, t53909, t53920, t53923, t53954)
}

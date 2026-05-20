//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta985 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3335;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3336;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3337;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3338;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3339;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta985<F: Float>(t141: F, t63253: F, t930: F, t18281: F, t2852: F, t606: F, t2908: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t51921: F, t51923: F, t63238: F, t63240: F, t63242: F, t63246: F, t63250: F, t4606: F, t918: F, t15107: F, t15110: F, t128: F, t63248: F, t904: F, t18943: F, t689: F, t18938: F, t2850: F, t18936: F, t2258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63255, t63258, t63260, t63262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3335::<F>(t141, t63253, t930, t18281, t2852, t606, t2908, t51909, t51911, t51913, t51915, t51917, t51921, t51923, t63238, t63240, t63242, t63246, t63250);
        let (t63266, t63268, t63274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3336::<F>(t4606, t918, t15107, t15110, t128, t63248, t904);
        let t63276 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3337::<F>(t18943, t689);
        let t63278 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3338::<F>(t18938, t689);
        let t63281 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3339::<F>(t128, t2850, t63258);
        let (t63283, t63285) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3340::<F>(t18936, t2258, t128, t2850);
    (t63255, t63258, t63260, t63262, t63266, t63268, t63274, t63276, t63278, t63281, t63283, t63285)
}

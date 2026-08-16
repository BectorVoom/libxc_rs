//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta985 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3335;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3336;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3337;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3338;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3339;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta985(t141: f64, t63253: f64, t930: f64, t18281: f64, t2852: f64, t606: f64, t2908: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t51921: f64, t51923: f64, t63238: f64, t63240: f64, t63242: f64, t63246: f64, t63250: f64, t4606: f64, t918: f64, t15107: f64, t15110: f64, t128: f64, t63248: f64, t904: f64, t18943: f64, t689: f64, t18938: f64, t2850: f64, t18936: f64, t2258: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63255, t63258, t63260, t63262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3335(t141, t63253, t930, t18281, t2852, t606, t2908, t51909, t51911, t51913, t51915, t51917, t51921, t51923, t63238, t63240, t63242, t63246, t63250);
        let (t63266, t63268, t63274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3336(t4606, t918, t15107, t15110, t128, t63248, t904);
        let t63276 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3337(t18943, t689);
        let t63278 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3338(t18938, t689);
        let t63281 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3339(t128, t2850, t63258);
        let (t63283, t63285) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3340(t18936, t2258, t128, t2850);
    (t63255, t63258, t63260, t63262, t63266, t63268, t63274, t63276, t63278, t63281, t63283, t63285)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta989 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3360;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3361;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3362;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3363;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3364;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3365;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta989(t128: f64, t63236: f64, t904: f64, t18931: f64, t689: f64, t63244: f64, t52011: f64, t52018: f64, t60927: f64, t52033: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t63359: f64, t63361: f64, t63366: f64, t42518: f64, t4606: f64, t2897: f64, t51957: f64, t52110: f64, t41329: f64, t51967: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t63299: f64, t63304: f64, t63308: f64, t41361: f64, t41363: f64, t51973: f64, t51978: f64, t63325: f64, t63328: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63369 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3360(t128, t63236, t904);
        let t63371 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3361(t18931, t689);
        let t63374 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3362(t128, t63244, t904);
        let (t63377, t63380) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3363(t52011, t52018, t60927, t52033, t63336, t63338, t63340, t63342, t63346, t63351, t63355, t63359, t63361, t63366, t63369, t63371, t63374);
        let (t63393, t63395, t63396, t63399) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3364(t42518, t52011, t60927, t4606, t2897, t51957, t52110);
        let t63412 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3365(t41329, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t63426 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t63440 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3367(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
    (t63369, t63371, t63374, t63377, t63380, t63393, t63395, t63396, t63399, t63412, t63426, t63440)
}

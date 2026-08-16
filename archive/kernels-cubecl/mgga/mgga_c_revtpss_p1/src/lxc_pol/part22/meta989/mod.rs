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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3360;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3361;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3362;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3363;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3364;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3365;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3367;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta989<F: Float>(t128: F, t63236: F, t904: F, t18931: F, t689: F, t63244: F, t52011: F, t52018: F, t60927: F, t52033: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t63359: F, t63361: F, t63366: F, t42518: F, t4606: F, t2897: F, t51957: F, t52110: F, t41329: F, t51967: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t63299: F, t63304: F, t63308: F, t41361: F, t41363: F, t51973: F, t51978: F, t63325: F, t63328: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t63369 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3360::<F>(t128, t63236, t904);
        let t63371 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3361::<F>(t18931, t689);
        let t63374 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3362::<F>(t128, t63244, t904);
        let (t63377, t63380) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3363::<F>(t52011, t52018, t60927, t52033, t63336, t63338, t63340, t63342, t63346, t63351, t63355, t63359, t63361, t63366, t63369, t63371, t63374);
        let (t63393, t63395, t63396, t63399) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3364::<F>(t42518, t52011, t60927, t4606, t2897, t51957, t52110);
        let t63412 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3365::<F>(t41329, t51967, t63274, t63276, t63278, t63281, t63285, t63290, t63293, t63299, t63304, t63308);
        let t63426 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3366::<F>(t41361, t41363, t51973, t51978, t63325, t63328, t63336, t63338, t63340, t63342, t63346, t63351, t63355);
        let t63440 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3367::<F>(t52033, t52035, t52037, t52039, t52041, t52045, t63359, t63361, t63366, t63369, t63371, t63374);
    (t63369, t63371, t63374, t63377, t63380, t63393, t63395, t63396, t63399, t63412, t63426, t63440)
}

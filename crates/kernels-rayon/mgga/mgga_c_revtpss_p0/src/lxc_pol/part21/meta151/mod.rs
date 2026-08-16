//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta151 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk961;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk962;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk963;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk964;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk965;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk966;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk967;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk968;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk969;
use chunk9::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk970;
use chunk10::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk971;
use chunk11::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta151(t3551: f64, t459: f64, t1203: f64, t1208: f64, t487: f64, t1204: f64, t1207: f64, t458: f64, t456: f64, t1214: f64, t1211: f64, t1209: f64, t1269: f64, t1294: f64, t1277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3552 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk961(t3551, t459);
        let t3555 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk962(t1203, t1208);
        let t3556 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk963(t3555, t487);
        let t3561 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk964(t1204, t487);
        let t3565 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk965(t1207, t458);
        let t3566 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk966(t3565, t456);
        let t3567 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk967(t3566, t487);
        let t3568 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk968(t1214);
        let t3569 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk969(t1211, t3568);
        let t3572 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk970(t1209, t1269);
        let t3575 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk971(t1214, t1294);
        let t3576 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk972(t1277, t3575);
    (t3552, t3555, t3556, t3561, t3565, t3566, t3567, t3568, t3569, t3572, t3575, t3576)
}

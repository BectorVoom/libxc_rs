//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta150 (260520-c91 hierarchical CSE).
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

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk988;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk989;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk990;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk991;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk992;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk993;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk994;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk995;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk996;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk997;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk998;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta150<F: Float>(t3551: F, t459: F, t1203: F, t1208: F, t487: F, t1204: F, t1207: F, t458: F, t456: F, t1214: F, t1211: F, t1209: F, t1269: F, t1294: F, t1277: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t3552 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk988::<F>(t3551, t459);
        let t3555 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk989::<F>(t1203, t1208);
        let t3556 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk990::<F>(t3555, t487);
        let t3561 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk991::<F>(t1204, t487);
        let t3565 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk992::<F>(t1207, t458);
        let t3566 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk993::<F>(t3565, t456);
        let t3567 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk994::<F>(t3566, t487);
        let t3568 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk995::<F>(t1214);
        let t3569 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk996::<F>(t1211, t3568);
        let t3572 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk997::<F>(t1209, t1269);
        let t3575 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk998::<F>(t1214, t1294);
        let t3576 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk999::<F>(t1277, t3575);
    (t3552, t3555, t3556, t3561, t3565, t3566, t3567, t3568, t3569, t3572, t3575, t3576)
}

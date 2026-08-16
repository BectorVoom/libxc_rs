//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta154 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1027;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1028;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1029;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1030;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1031;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1032;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1033;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta154(t1122: f64, t3634: f64, t247: f64, t1261: f64, t1264: f64, t3372: f64, t3368: f64, t1230: f64, t1260: f64, t225: f64, t3552: f64, t480: f64, t371: f64, t482: f64, t676: f64, t481: f64, t1231: f64, t1256: f64, t1247: f64, t1266: f64, t3591: f64, t3600: f64, t3606: f64, t3610: f64, t3613: f64, t3620: f64, t3625: f64, t3631: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3636 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1027(t1122, t3634, t247);
        let (t3637, t3640) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1028(t1261, t3636, t1264, t3372, t247);
        let t3644 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1029(t1264, t3368, t247);
        let t3647 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1030(t1230, t1260);
        let t3650 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1031(t225, t3552);
        let (t3651, t3655) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1032(t3650, t480, t371, t482, t676);
        let (t3657, t3658, t3660) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1033(t3655, t481, t1231, t1256, t1247, t1261, t1266, t3591, t3600, t3606, t3610, t3613, t3620, t3625, t3631, t3637, t3640, t3644, t3647, t3651, t484);
    (t3636, t3637, t3640, t3644, t3647, t3650, t3651, t3655, t3657, t3658, t3660)
}

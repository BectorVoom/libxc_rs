//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta129 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk840;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk841;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk842;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk843;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk844;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk845;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk846;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk847;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta129(t3617: f64, t66: f64, t474: f64, t479: f64, t3089: f64, t1285: f64, t1264: f64, t828: f64, t1248: f64, t73: f64, t1121: f64, t471: f64, t606: f64, t126: f64, t1263: f64, t1122: f64, t247: f64, t1261: f64, t1230: f64, t1260: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3618, t3623) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk840(t3617, t66, t474, t479);
        let t3624 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk841(t3089, t3623);
        let t3625 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk842(t1285, t3624);
        let t3626 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk843(t1264, t828);
        let (t3627, t3628, t3629) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk844(t1248, t73, t1121, t471, t606);
        let t3634 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk845(t126, t1263);
        let t3636 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk846(t1122, t3634, t247);
        let (t3637, t3647) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk847(t1261, t3636, t1230, t1260);
    (t3618, t3623, t3624, t3625, t3626, t3627, t3628, t3629, t3634, t3636, t3637, t3647)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta155 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk989;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk990;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk991;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk992;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk993;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk994;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk995;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta155(t1248: f64, t73: f64, t1121: f64, t471: f64, t606: f64, t3626: f64, t126: f64, t1263: f64, t1122: f64, t247: f64, t1261: f64, t1264: f64, t3372: f64, t3368: f64, t1230: f64, t1260: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3627, t3628, t3629) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk989(t1248, t73, t1121, t471, t606);
        let t3630 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk990(t3627, t3629);
        let t3631 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk991(t3626, t3630);
        let t3634 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk992(t126, t1263);
        let t3636 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk993(t1122, t3634, t247);
        let (t3637, t3640) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk994(t1261, t3636, t1264, t3372, t247);
        let t3644 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk995(t1264, t3368, t247);
        let t3647 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk996(t1230, t1260);
    (t3627, t3628, t3629, t3630, t3631, t3634, t3636, t3637, t3640, t3644, t3647)
}

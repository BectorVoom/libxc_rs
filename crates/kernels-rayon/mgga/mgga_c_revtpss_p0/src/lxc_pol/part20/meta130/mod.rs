//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta130 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk737;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk738;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk739;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk740;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk741;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk742;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk743;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta130(t3551: f64, t459: f64, t1203: f64, t1208: f64, t487: f64, t1204: f64, t1207: f64, t458: f64, t456: f64, t1214: f64, t1211: f64, t1209: f64, t1269: f64, t1294: f64, t1277: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3552 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk737(t3551, t459);
        let t3555 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk738(t1203, t1208);
        let t3556 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk739(t3555, t487);
        let (t3561, t3565, t3566) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk740(t1204, t487, t1207, t458, t456);
        let (t3567, t3568) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk741(t3566, t487, t1214);
        let (t3569, t3572) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk742(t1211, t3568, t1209, t1269);
        let (t3575, t3576) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk743(t1214, t1294, t1277);
    (t3552, t3555, t3556, t3561, t3565, t3566, t3567, t3568, t3569, t3572, t3575, t3576)
}

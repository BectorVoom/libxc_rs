//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta167 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk890;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk891;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk892;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk893;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk894;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk895;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta167(t187: f64, t9363: f64, t2490: f64, t737: f64, t2492: f64, t744: f64, t185: f64, t2494: f64, t1340: f64, t2516: f64, t4038: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64, t738: f64, t745: f64, t1320: f64, t3853: f64, t123: f64, t147: f64, t9291: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9365, t9367) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk890(t187, t9363, t2490, t737);
        let t9368 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk891(t2492, t744);
        let t9371 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk892(t185, t2494);
        let t9372 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk893(t9367, t9368, t9371);
        let (t9374, t9376, t9385) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk894(t1340, t9372, t2516, t4038, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
        let t9387 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk895(t738, t745, t9385);
        let (t9389, t9391, t9394) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk896(t1340, t9387, t1320, t3853, t123, t147, t9291);
    (t9365, t9367, t9368, t9371, t9372, t9374, t9376, t9385, t9387, t9389, t9391, t9394)
}

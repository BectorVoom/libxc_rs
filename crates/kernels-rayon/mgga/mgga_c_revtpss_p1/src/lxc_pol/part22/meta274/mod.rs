//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta274 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1668;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1669;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1670;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1671;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1672;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1673;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1674;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta274(t2564: f64, t2567: f64, t268: f64, t675: f64, t30: f64, t525: f64, t2: f64, t22: f64, t33: f64, t527: f64, t2490: f64, t737: f64, t2492: f64, t744: f64, t185: f64, t2494: f64, t1340: f64, t2516: f64, t4038: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64, t738: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t9333 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1668(t2564, t2567, t268, t675);
        let (t9335, t9342) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1669(t30, t525, t2, t22);
        let (t9350, t9367) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1670(t33, t527, t2490, t737);
        let t9368 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1671(t2492, t744);
        let t9371 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1672(t185, t2494);
        let t9372 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1673(t9367, t9368, t9371);
        let (t9374, t9375, t9385) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1674(t1340, t9372, t2516, t4038, t9283, t9286, t9289, t9292, t9296, t9298, t9300, t9303);
        let t9387 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1675(t738, t745, t9385);
    (t9333, t9335, t9342, t9350, t9367, t9368, t9371, t9372, t9374, t9375, t9385, t9387)
}

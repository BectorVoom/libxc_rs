//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk917;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk918;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk919;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk920;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta192(t1376: f64, t9789: f64, t235: f64, t4086: f64, t2453: f64, t240: f64, t2712: f64, t785: f64, t9731: f64, t225: f64, t4062: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64, t220: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9791, t9792, t9793) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk917(t1376, t9789, t235, t4086, t2453);
        let t9794 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk918(t240, t2712);
        let t9801 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk919(t785, t9731);
        let (t9802, t9804, t9816) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk920(t225, t9801, t4062, t1386, t2482, t814);
        let t9818 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk921(t136, t1412, t220);
    (t9791, t9792, t9793, t9794, t9801, t9802, t9804, t9816, t9818)
}

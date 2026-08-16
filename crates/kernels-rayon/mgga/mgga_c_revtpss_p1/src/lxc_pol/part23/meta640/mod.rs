//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta640 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2348;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2349;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2350;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2351;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2352;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2353;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2354;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2355;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta640(t192: f64, t268: f64, t9450: f64, t9501: f64, t9476: f64, t9508: f64, t2582: f64, t2584: f64, t39480: f64, t2519: f64, t9306: f64, t9518: f64, t9540: f64, t681: f64, t702: f64, t793: f64, t215: f64, t2564: f64, t2567: f64, t2566: f64, t2576: f64, t9311: f64, t9313: f64, t2580: f64, t2583: f64, t130: f64, t39525: f64, t2563: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39762, t39764, t39768, t39770, t39773) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2348(t192, t268, t9450, t9501, t9476, t9508, t2582, t2584, t39480);
        let t39783 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2349(t2519, t268, t9306);
        let t39786 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2350(t268, t9518, t9540);
        let t39791 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2351(t268, t681, t702, t793);
        let t39795 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2352(t215, t2564, t2567, t268);
        let t39799 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2353(t2566, t2576, t9311, t9313);
        let t39807 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2354(t2580, t2583, t130, t39525);
        let t39813 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2355(t130, t2563, t2580, t39525, t9313);
    (t39762, t39764, t39768, t39770, t39773, t39783, t39786, t39791, t39795, t39799, t39807, t39813)
}

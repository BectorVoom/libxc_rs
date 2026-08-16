//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta719 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2758;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2759;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2760;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2761;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2762;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta719(t192: f64, t268: f64, t9450: f64, t9501: f64, t2258: f64, t2609: f64, t706: f64, t9476: f64, t9508: f64, t2582: f64, t2584: f64, t39480: f64, t10587: f64, t2516: f64, t2401: f64, t2519: f64, t9306: f64, t9518: f64, t9540: f64, t681: f64, t702: f64, t793: f64, t215: f64, t2564: f64, t2567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39762, t39764, t39766, t39768, t39770, t39773) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2758(t192, t268, t9450, t9501, t2258, t2609, t706, t9476, t9508, t2582, t2584, t39480);
        let (t39774, t39779, t39783) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2759(t10587, t2516, t2401, t2609, t2519, t268, t9306);
        let t39786 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2760(t268, t9518, t9540);
        let t39791 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2761(t268, t681, t702, t793);
        let t39795 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2762(t215, t2564, t2567, t268);
    (t39762, t39764, t39766, t39768, t39770, t39773, t39774, t39779, t39783, t39786, t39791, t39795)
}

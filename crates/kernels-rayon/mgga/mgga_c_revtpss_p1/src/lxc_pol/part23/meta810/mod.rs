//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2652;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2653;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2654;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta810(t2435: f64, t6093: f64, t6097: f64, t6101: f64, t2439: f64, t6132: f64, t6135: f64, t19013: f64, t698: f64, t19016: f64, t6138: f64, t18960: f64, t18963: f64, t18966: f64, t19077: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t63453 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2652(t2435, t6093);
        let t63459 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2653(t2435, t6097);
        let t63464 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2654(t2435, t6101);
        let (t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t63610) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2655(t2439, t6132, t6135, t19013, t698, t19016, t6138, t18960, t18963, t18966, t19077, t914);
    (t63453, t63459, t63464, t63533, t63538, t63541, t63543, t63545, t63547, t63549, t63551, t63610)
}

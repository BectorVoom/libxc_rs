//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta931 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3050;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3051;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3052;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3053;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3054;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta931(t20297: f64, t4186: f64, t128: f64, t3360: f64, t24228: f64, t606: f64, t16724: f64, t5825: f64, t22671: f64, t3362: f64, t18281: f64, t5046: f64, t6421: f64, t1120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81169, t81171) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3050(t20297, t4186, t128, t3360);
        let (t81173, t81175) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3051(t24228, t606, t128, t3360);
        let (t81177, t81179) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3052(t16724, t5825, t128, t3360);
        let (t81182, t81184) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3053(t22671, t3362, t606, t128, t3360);
        let (t81186, t81188) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3054(t18281, t5046, t128, t3360);
        let (t81190, t81192) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3055(t4186, t6421, t1120, t128);
    (t81169, t81171, t81173, t81175, t81177, t81179, t81182, t81184, t81186, t81188, t81190, t81192)
}

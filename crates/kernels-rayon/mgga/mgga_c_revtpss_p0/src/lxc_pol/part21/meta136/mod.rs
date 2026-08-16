//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta136 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk877;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk878;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk879;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk880;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk881;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta136(t1026: f64, t127: f64, t371: f64, t1025: f64, t3075: f64, t373: f64, t372: f64, t225: f64, t3046: f64, t366: f64, t362: f64, t40: f64, t611: f64, t361: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3215 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk877(t1026, t127, t371);
        let (t3216, t3218, t3220) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk878(t1025, t3215, t3075, t373, t371, t372);
        let t3223 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk879(t225, t3046);
        let t3224 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk880(t3223, t366);
        let (t3229, t3230) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk881(t362, t40, t611, t361);
    (t3215, t3216, t3218, t3220, t3223, t3224, t3229, t3230)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2563;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2564;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2565;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta636(t1145: f64, t20272: f64, t141: f64, t6461: f64, t698: f64, t6464: f64, t6467: f64, t6422: f64, t689: f64, t6426: f64, t6430: f64, t1120: f64, t128: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20273, t20274, t20276, t20278, t20280, t20283) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2563(t1145, t20272, t141, t6461, t698, t6464, t6467, t6422, t689);
        let t20285 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2564(t6426, t689);
        let t20287 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2565(t6430, t689);
        let (t20289, t20290) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2566(t1120, t20272, t128);
    (t20273, t20274, t20276, t20278, t20280, t20283, t20285, t20287, t20289, t20290)
}

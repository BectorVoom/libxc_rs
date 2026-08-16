//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta92 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk597;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk598;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk599;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk600;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk601;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk602;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk603;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk604;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta92(t1450: f64, t2034: f64, t2014: f64, t117: f64, t1936: f64, t572: f64, t55: f64, t61: f64, t68: f64, t72: f64, t1927: f64, t5: f64, t1923: f64, t265: f64, t393: f64, t1995: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2035 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk597(t1450, t2034);
        let (t2036, t2042) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk598(t2014, t2035, t117, t1936);
        let (t2044, t2121) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk599(t2042, t572, t55, t61, t68);
        let t2122 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk600(t2121, t72);
        let t2123 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk601(t1927, t2122);
        let t2126 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk602(t5, t1923, t2123);
        let t2127 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk603(t117, t2126);
        let t2129 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk604(t265, t393, t1995);
    (t2035, t2036, t2042, t2044, t2121, t2122, t2123, t2126, t2127, t2129)
}

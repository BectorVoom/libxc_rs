//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta582 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2203;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2204;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2205;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2206;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta582(t11341: f64, t23470: f64, t141: f64, t22671: f64, t905: f64, t930: f64, t11142: f64, t128: f64, t11150: f64, t22688: f64, t2850: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23471, t23472, t23474) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2203(t11341, t23470, t141, t22671, t905);
        let (t23475, t23476, t23478, t23479) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2204(t23474, t930, t141, t11142, t23470, t128);
        let t23481 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2205(t11150, t22688);
        let (t23482, t23483) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2206(t23481, t2850, t128);
        let t23485 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2207(t22688, t2852);
    (t23471, t23472, t23474, t23475, t23476, t23478, t23479, t23481, t23482, t23483, t23485)
}

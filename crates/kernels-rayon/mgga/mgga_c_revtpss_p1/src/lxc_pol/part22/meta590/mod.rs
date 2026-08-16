//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2468;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2469;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2470;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2471;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta590(t4186: f64, t4578: f64, t904: f64, t128: f64, t6101: f64, t689: f64, t2852: f64, t5825: f64, t606: f64, t2850: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18930, t18931, t18932) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2468(t4186, t4578, t904, t128);
        let t18934 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2469(t6101, t689);
        let (t18936, t18937, t18938, t18939) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2470(t2852, t5825, t606, t2850, t128);
        let (t18941, t18942, t18943, t18944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2471(t2857, t5825, t606, t904, t128);
    (t18930, t18931, t18932, t18934, t18936, t18937, t18938, t18939, t18941, t18942, t18943, t18944)
}

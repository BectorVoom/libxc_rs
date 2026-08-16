//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2205;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2206;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta486(t1651: f64, t3133: f64, t1045: f64, t3117: f64, t12167: f64, t15905: f64, t11631: f64, t3151: f64, t15907: f64, t3057: f64, t380: f64, t3088: f64, t370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16076, t16077, t16078, t16081) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2205(t1651, t3133, t1045, t3117, t12167, t15905);
        let (t16082, t16083, t16084, t16087, t16088) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2206(t11631, t3151, t15907, t3117, t3057, t380, t3088, t370);
        let t16089 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2207(t16087, t16088);
    (t16076, t16077, t16078, t16081, t16082, t16083, t16084, t16087, t16088, t16089)
}

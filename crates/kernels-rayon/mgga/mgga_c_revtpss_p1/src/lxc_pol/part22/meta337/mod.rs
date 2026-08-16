//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1795;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1796;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1797;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1798;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1799;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta337(t262: f64, t775: f64, t3335: f64, t389: f64, t1077: f64, t225: f64, t1071: f64, t3046: f64, t268: f64, t271: f64, t7021: f64, t2435: f64, t907: f64, t2854: f64, t689: f64, t2859: f64, t2863: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11088, t11108, t11119, t11120, t11121, t11128, t11132) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1795(t262, t775, t3335, t389, t1077, t225, t1071, t3046, t268, t271, t7021);
        let (t11133, t11134) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1796(t11132, t2435, t907);
        let t11136 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1797(t2854, t689);
        let t11138 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1798(t2859, t689);
        let t11140 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1799(t2863, t689);
    (t11088, t11108, t11119, t11120, t11121, t11128, t11132, t11133, t11134, t11136, t11138, t11140)
}

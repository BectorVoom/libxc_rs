//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2339;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2340;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta637(t2452: f64, t588: f64, t258: f64, t2454: f64, t2455: f64, t39494: f64, t10985: f64, t11018: f64, t10541: f64, t2453: f64, t231: f64, t268: f64, t2798: f64, t793: f64, t836: f64, t14545: f64, t251: f64, t786: f64, t4503: f64, t860: f64, t10115: f64, t883: f64, t2710: f64, t2793: f64, t2804: f64, t874: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39552, t39554, t39557, t39558, t39575, t39581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2339(t2452, t588, t258, t2454, t2455, t39494, t10985, t11018, t10541, t2453, t231, t268, t2798, t793, t836);
        let (t39597, t39598, t39609, t39624, t39633, t39635) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2340(t14545, t251, t786, t4503, t860, t10115, t883, t2710, t2793, t39494, t2804, t874, t9288);
    (t39552, t39554, t39557, t39558, t39575, t39581, t39597, t39598, t39609, t39624, t39633, t39635)
}

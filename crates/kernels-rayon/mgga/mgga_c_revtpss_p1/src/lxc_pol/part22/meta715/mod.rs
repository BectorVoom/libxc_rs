//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta715 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2747;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2748;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2749;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta715(t215: f64, t2722: f64, t231: f64, t268: f64, t2798: f64, t2645: f64, t14545: f64, t251: f64, t4503: f64, t860: f64, t786: f64, t10115: f64, t883: f64, t2710: f64, t2793: f64, t39494: f64, t2804: f64, t874: f64, t9288: f64, t10535: f64, t281: f64, t68: f64, t211: f64, t9644: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39583, t39586, t39595, t39597, t39608, t39609, t39624) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2747(t215, t2722, t231, t268, t2798, t2645, t14545, t251, t4503, t860, t786, t10115, t883);
        let (t39633, t39635, t39640, t39643) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2748(t2710, t2793, t39494, t2804, t874, t9288, t10535, t231, t2645, t281, t68, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2749(t209, t39643);
    (t39583, t39586, t39595, t39597, t39608, t39609, t39624, t39633, t39635, t39640, t39644)
}

//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2330;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2331;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2332;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2333;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta603(t2710: f64, t2793: f64, t39494: f64, t2804: f64, t874: f64, t9288: f64, t10535: f64, t231: f64, t2645: f64, t281: f64, t68: f64, t211: f64, t9644: f64, t209: f64, t234: f64, t251: f64, t268: f64, t8779: f64, t39497: f64, t875: f64, t10530: f64, t2723: f64, t39583: f64, t2798: f64, t39599: f64, t624: f64, t836: f64, t2722: f64, t10529: f64, t2453: f64, t10523: f64, t10542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39633, t39635, t39640, t39643) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2330(t2710, t2793, t39494, t2804, t874, t9288, t10535, t231, t2645, t281, t68, t211, t9644);
        let t39644 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2331(t209, t39643);
        let (t39649, t39652, t39662, t39668) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2332(t234, t251, t268, t39644, t8779, t39497, t874, t875, t10530, t2723, t39583, t231, t2798, t39599);
        let (t39673, t39678, t39683, t39685) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2333(t10535, t231, t281, t624, t836, t2722, t68, t10529, t2453, t2723, t10523, t10542);
    (t39633, t39635, t39640, t39644, t39649, t39652, t39662, t39668, t39673, t39678, t39683, t39685)
}

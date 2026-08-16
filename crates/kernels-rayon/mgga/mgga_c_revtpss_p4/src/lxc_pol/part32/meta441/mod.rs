//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1606;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta441(t3957: f64, t6884: f64, t124: f64, t21969: f64, t800: f64, t6850: f64, t9744: f64, t125: f64, t6861: f64, t3936: f64, t9835: f64, t1414: f64, t828: f64, t221: f64, t3979: f64, t6816: f64, t3978: f64, t3989: f64, t6880: f64, t22025: f64, t543: f64, t3992: f64, t2661: f64, t1370: f64, t13779: f64, t13781: f64, t13797: f64, t1410: f64, t5671: f64, t9735: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22038, t22041, t22044, t22046, t22048, t22052) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1606(t3957, t6884, t124, t21969, t800, t6850, t9744, t125, t6861, t3936, t9835, t1414, t828);
        let (t22056, t22061, t22065) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1607(t221, t3979, t6816, t3978, t3989, t6880, t22025, t543, t3992, t2661, t1370, t13779, t13781, t13797, t1410, t22038, t22041, t22044, t22048, t22052, t5671, t9735);
    (t22041, t22046, t22048, t22052, t22056, t22061, t22065)
}

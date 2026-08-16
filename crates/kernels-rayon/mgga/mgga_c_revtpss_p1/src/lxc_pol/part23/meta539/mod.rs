//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta539 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2085;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta539(t22046: f64, t3936: f64, t9835: f64, t1414: f64, t21969: f64, t828: f64, t221: f64, t3979: f64, t6816: f64, t3978: f64, t3989: f64, t6880: f64, t22025: f64, t543: f64, t3992: f64, t2661: f64, t1370: f64, t13779: f64, t13781: f64, t13797: f64, t1410: f64, t22038: f64, t22041: f64, t22044: f64, t5671: f64, t9735: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22048, t22052, t22056, t22057, t22059) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2085(t22046, t3936, t9835, t1414, t21969, t828, t221, t3979, t6816, t3978, t3989, t6880);
        let (t22061, t22062, t22063, t22065) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2086(t22025, t543, t3992, t2661, t1370, t13779, t13781, t13797, t1410, t22038, t22041, t22044, t22048, t22052, t22057, t22059, t5671, t9735);
    (t22048, t22052, t22056, t22057, t22059, t22061, t22062, t22063, t22065)
}

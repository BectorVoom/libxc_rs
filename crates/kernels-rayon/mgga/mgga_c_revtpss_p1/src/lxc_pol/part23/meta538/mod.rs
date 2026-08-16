//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta538 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2083;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta538(t22025: f64, t4003: f64, t9934: f64, t2661: f64, t3989: f64, t6856: f64, t13762: f64, t13763: f64, t13765: f64, t13772: f64, t13778: f64, t22023: f64, t9711: f64, t9712: f64, t9725: f64, t9729: f64, t3957: f64, t6884: f64, t124: f64, t21969: f64, t800: f64, t6850: f64, t9744: f64, t125: f64, t6861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22026, t22027, t22028, t22030, t22035) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2083(t22025, t4003, t9934, t2661, t3989, t6856, t13762, t13763, t13765, t13772, t13778, t22023, t9711, t9712, t9725, t9729);
        let (t22038, t22041, t22044, t22046) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2084(t3957, t6884, t124, t21969, t800, t6850, t9744, t125, t6861);
    (t22026, t22027, t22028, t22030, t22035, t22038, t22041, t22044, t22046)
}

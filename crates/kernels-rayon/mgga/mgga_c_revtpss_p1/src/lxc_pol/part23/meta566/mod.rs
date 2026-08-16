//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta566 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2143;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2144;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2145;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta566(t22813: f64, t828: f64, t9942: f64, t1414: f64, t22809: f64, t22079: f64, t3936: f64, t6869: f64, t13790: f64, t5673: f64, t1883: f64, t22074: f64, t13765: f64, t13779: f64, t13781: f64, t1410: f64, t22023: f64, t22028: f64, t22030: f64, t3934: f64, t5671: f64, t9711: f64, t9725: f64, t9729: f64, t1868: f64, t4003: f64, t22046: f64, t124: f64, t800: f64, t6816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22815, t22822, t22829, t22833, t22837) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2143(t22813, t828, t9942, t1414, t22809, t22079, t3936, t6869, t13790, t5673, t1883, t22074);
        let t22840 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2144(t13765, t13779, t13781, t1410, t22023, t22028, t22030, t22815, t22822, t22829, t22833, t22837, t3934, t5671, t9711, t9725, t9729);
        let (t22841, t22843, t22848, t22849, t22852) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2145(t1868, t4003, t22046, t3936, t124, t22809, t800, t6816);
    (t22815, t22822, t22829, t22833, t22837, t22840, t22841, t22843, t22848, t22849, t22852)
}

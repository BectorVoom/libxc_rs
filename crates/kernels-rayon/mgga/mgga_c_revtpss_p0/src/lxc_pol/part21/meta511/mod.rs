//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2138;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2139;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta511(t11631: f64, t3151: f64, t15907: f64, t3117: f64, t3057: f64, t380: f64, t3088: f64, t370: f64, t4757: f64, t906: f64, t3092: f64, t994: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16082, t16083, t16084, t16087, t16088) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2138(t11631, t3151, t15907, t3117, t3057, t380, t3088, t370);
        let t16089 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2139(t16087, t16088);
        let (t16090, t16091, t16094, t16095) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2140(t4757, t906, t3092, t380, t994, t16088);
    (t16082, t16083, t16084, t16087, t16088, t16089, t16090, t16091, t16094, t16095)
}

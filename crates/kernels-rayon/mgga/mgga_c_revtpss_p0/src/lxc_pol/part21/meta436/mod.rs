//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta436 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1946;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1947;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta436(t3936: f64, t5674: f64, t9810: f64, t125: f64, t5591: f64, t1399: f64, t4057: f64, t5704: f64, t1872: f64, t9818: f64, t9816: f64, t5706: f64, t9962: f64, t13944: f64, t5673: f64, t5675: f64, t9955: f64, t9956: f64, t4000: f64, t820: f64, t844: f64, t5677: f64, t3934: f64, t5671: f64, t9847: f64, t9896: f64, t9906: f64, t9910: f64, t9919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13967, t13975, t13977, t13981, t13985, t13987, t13988) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1946(t3936, t5674, t9810, t125, t5591, t1399, t4057, t5704, t1872, t9818, t9816, t5706, t9962);
        let (t13991, t13995, t13999) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1947(t13944, t5673, t5675, t5674, t9955, t9956, t4000, t820, t844);
        let t14002 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1948(t13999, t5677, t13967, t13977, t13981, t13987, t13988, t13991, t13995, t3934, t5671, t9847, t9896, t9906, t9910, t9919);
    (t13967, t13975, t13977, t13981, t13985, t13991, t13995, t13999, t14002)
}

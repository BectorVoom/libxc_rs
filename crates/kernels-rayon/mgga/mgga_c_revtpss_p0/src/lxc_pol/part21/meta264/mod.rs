//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1464;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1465;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1466;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta264(t225: f64, t9801: f64, t4062: f64, t125: f64, t4056: f64, t3936: f64, t3938: f64, t3889: f64, t543: f64, t3937: f64, t1386: f64, t2482: f64, t814: f64, t136: f64, t1412: f64, t220: f64, t124: f64, t1398: f64, t1410: f64, t3934: f64, t9757: f64, t9762: f64, t9766: f64, t9771: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t9796: f64, t9799: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9802, t9804, t9807, t9810) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1464(t225, t9801, t4062, t125, t4056, t3936, t3938, t3889, t543);
        let (t9812, t9816) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1465(t3936, t3937, t9810, t1386, t2482, t814);
        let (t9817, t9818) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1466(t136, t1412, t220);
        let (t9819, t9821, t9822, t9824) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1467(t124, t1398, t3938, t9818, t9816, t1410, t3934, t9757, t9762, t9766, t9771, t9776, t9780, t9786, t9791, t9796, t9799, t9804, t9807, t9812);
    (t9802, t9804, t9807, t9810, t9812, t9816, t9817, t9818, t9819, t9821, t9822, t9824)
}

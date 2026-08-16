//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1464;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1465;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1466;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta264<F: Float>(t225: F, t9801: F, t4062: F, t125: F, t4056: F, t3936: F, t3938: F, t3889: F, t543: F, t3937: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t124: F, t1398: F, t1410: F, t3934: F, t9757: F, t9762: F, t9766: F, t9771: F, t9776: F, t9780: F, t9786: F, t9791: F, t9796: F, t9799: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9802, t9804, t9807, t9810) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1464::<F>(t225, t9801, t4062, t125, t4056, t3936, t3938, t3889, t543);
        let (t9812, t9816) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1465::<F>(t3936, t3937, t9810, t1386, t2482, t814);
        let (t9817, t9818) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1466::<F>(t136, t1412, t220);
        let (t9819, t9821, t9822, t9824) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1467::<F>(t124, t1398, t3938, t9818, t9816, t1410, t3934, t9757, t9762, t9766, t9771, t9776, t9780, t9786, t9791, t9796, t9799, t9804, t9807, t9812);
    (t9802, t9804, t9807, t9810, t9812, t9816, t9817, t9818, t9819, t9821, t9822, t9824)
}

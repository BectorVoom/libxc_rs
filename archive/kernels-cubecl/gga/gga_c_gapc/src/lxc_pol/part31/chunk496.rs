//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 496/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk496<F: Float>(t2315: F, t2801: F, t330: F, t197: F, t617: F, t953: F, t1793: F, t889: F, t2233: F, t2636: F, t1686: F, t325: F) -> (F, F, F, F, F) {
    let t2802 = t2801 * t2315;
    let t2803 = t330 * t2802;
    let t2804 = t197 * t2803;
    let t2807 = t617 * t953;
    let t2810 = t889 * t1793;
    let t2811 = t2636 * t2233;
    let t2814 = t325 * t1686;
    (t2804, t2807, t2810, t2811, t2814)
}

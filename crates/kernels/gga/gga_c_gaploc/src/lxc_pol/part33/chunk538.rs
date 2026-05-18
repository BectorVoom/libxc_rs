//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 538/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk538<F: Float>(t501: F, t997: F, t1016: F, t605: F, t1012: F, t1628: F, t1589: F, t993: F, t1007: F, t2754: F, t600: F, t568: F) -> (F, F, F, F, F, F, F) {
    let t2798 = t997 * t501;
    let t2801 = t1016 * t605;
    let t2804 = t1628 * t1012;
    let t2807 = t1589 * t993;
    let t2810 = t1628 * t1007;
    let t2815 = t600 * t2754;
    let t2816 = t568 * t2815;
    (t2798, t2801, t2804, t2807, t2810, t2815, t2816)
}

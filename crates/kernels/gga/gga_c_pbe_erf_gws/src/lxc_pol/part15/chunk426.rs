//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 426/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk426<F: Float>(t1630: F, t644: F, t639: F, t1416: F, t643: F, t642: F, t212: F, t626: F) -> (F, F, F, F, F, F, F) {
    let t1631 = t1630 * t644;
    let t1632 = t639 * t1631;
    let t1633 = 16.0 / 135.0 * t1632;
    let t1634 = t643 * t1416;
    let t1635 = t642 * t1634;
    let t1637 = 4.0 / 45.0 * t639 * t1635;
    let t1639 = 1.0 / t212 / t626;
    (t1631, t1632, t1633, t1634, t1635, t1637, t1639)
}

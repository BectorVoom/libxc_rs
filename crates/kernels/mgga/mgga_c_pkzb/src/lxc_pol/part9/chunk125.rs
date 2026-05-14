//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 125/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk125<F: Float>(t218: F, t219: F, t344: F, t336: F, t339: F, t342: F) -> (F, F, F, F) {
    let t346 = t218 * t219 * t344;
    let t348 = 0.379785e1 * t339 + 0.8969e0 * t336 + 0.204775e0 * t342 + 0.123235e0 * t346;
    let t351 = 1.0 + 0.16081979498692535067e2 / t348;
    let t352 = f64::ln(t351);
    (t346, t348, t351, t352)
}

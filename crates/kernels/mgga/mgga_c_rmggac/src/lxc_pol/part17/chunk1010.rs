//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1010/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1010<F: Float>(t1763: F, t664: F, t2367: F, t570: F, t27048: F, t27176: F, t321: F, t333: F, t352: F, t36035: F, t41122: F, t46550: F, t46603: F, t46605: F, t46607: F, t46609: F, t46612: F, t46614: F, t5148: F, t5266: F) -> (F, F, F) {
    let t46622 = t664 * t1763;
    let t46626 = t2367 * t570;
    let t46633 = F::new(0.47896966807455234256e0) * t46603 - F::new(0.2993560425465952141e-1) * t46605 + F::new(0.44903406381989282115e-1) * t46607 + F::new(0.2993560425465952141e-1) * t46609 + F::new(0.2993560425465952141e-1) * t46612 - F::new(0.23948483403727617128e0) * t46614 - F::new(0.47896966807455234256e0) * t27176 * t46550 * t352 + t36035 + F::new(0.23948483403727617128e0) * t5266 * t41122 * t570 + F::new(0.35922725105591425692e0) * t27048 * t46622 * t321 - F::new(0.23948483403727617128e0) * t5148 * t46626 * t321 + F::new(0.23948483403727617128e0) * t5266 * t46626 * t333;
    (t46622, t46626, t46633)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1010/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1010(t1763: f64, t664: f64, t2367: f64, t570: f64, t27048: f64, t27176: f64, t321: f64, t333: f64, t352: f64, t36035: f64, t41122: f64, t46550: f64, t46603: f64, t46605: f64, t46607: f64, t46609: f64, t46612: f64, t46614: f64, t5148: f64, t5266: f64) -> (f64, f64, f64) {
    let t46622 = t664 * t1763;
    let t46626 = t2367 * t570;
    let t46633 = 0.47896966807455234256e0_f64 * t46603 - 0.2993560425465952141e-1_f64 * t46605 + 0.44903406381989282115e-1_f64 * t46607 + 0.2993560425465952141e-1_f64 * t46609 + 0.2993560425465952141e-1_f64 * t46612 - 0.23948483403727617128e0_f64 * t46614 - 0.47896966807455234256e0_f64 * t27176 * t46550 * t352 + t36035 + 0.23948483403727617128e0_f64 * t5266 * t41122 * t570 + 0.35922725105591425692e0_f64 * t27048 * t46622 * t321 - 0.23948483403727617128e0_f64 * t5148 * t46626 * t321 + 0.23948483403727617128e0_f64 * t5266 * t46626 * t333;
    (t46622, t46626, t46633)
}

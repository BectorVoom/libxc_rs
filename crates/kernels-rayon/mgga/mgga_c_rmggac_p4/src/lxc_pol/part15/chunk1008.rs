//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1008/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1008(t551: f64, t8957: f64, t1704: f64, t664: f64, t2367: f64, t6376: f64, t665: f64, t1743: f64, t2124: f64, t45166: f64, t5148: f64, t1652: f64, t27094: f64, t27101: f64, t305: f64, t326: f64, t333: f64, t352: f64, t41059: f64, t41439: f64, t46494: f64, t46550: f64, t4669: f64, t5155: f64, t5266: f64, t558: f64, t8975: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46563 = t8957 * t551;
    let t46575 = t664 * t1704;
    let t46582 = t2367 * t551;
    let t46586 = t665 * t6376;
    let t46589 = t2124 * t1743;
    let t46592 = t664 * t1743;
    let t46599 = t5148 * t45166;
    let t46601 = 0.11974241701863808564e0_f64 * t305 * t46563 + 0.11974241701863808564e0_f64 * t5266 * t46494 * t333 - 0.11974241701863808564e1_f64 * t27094 * t46550 * t333 - 0.35922725105591425692e0_f64 * t4669 * t41059 * t558 - 0.23948483403727617128e0_f64 * t27101 * t46575 * t352 - 0.23948483403727617128e0_f64 * t5148 * t8975 * t1652 - 0.23948483403727617128e0_f64 * t5148 * t46582 * t352 + t41439 - 0.59871208509319042821e-1_f64 * t326 * t46586 - 0.59871208509319042821e-1_f64 * t326 * t46589 + 0.23948483403727617128e0_f64 * t5155 * t46592 * t333 + 0.11974241701863808564e0_f64 * t5266 * t46592 * t352 + 0.5987120850931904282e-1_f64 * t46599;
    (t46563, t46575, t46582, t46586, t46589, t46592, t46601)
}

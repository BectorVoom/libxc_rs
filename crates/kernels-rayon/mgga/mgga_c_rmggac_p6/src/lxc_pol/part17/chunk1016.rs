//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1016/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1016(t46427: f64, t5148: f64, t2064: f64, t9908: f64, t46501: f64, t5259: f64, t40826: f64, t9704: f64, t1587: f64, t1614: f64, t25820: f64, t305: f64, t321: f64, t333: f64, t36058: f64, t40983: f64, t41059: f64, t45527: f64, t46575: f64, t46582: f64, t4669: f64, t46694: f64, t558: f64, t570: f64, t8975: f64) -> f64 {
    let t46702 = t5148 * t46427;
    let t46707 = t9908 * t2064;
    let t46710 = t5259 * t46501;
    let t46715 = t40826 * t9704;
    let t46734 = 0.2993560425465952141e-1_f64 * t46702 - 0.35922725105591425692e0_f64 * t4669 * t46694 * t321 - 0.79828278012425390427e-1_f64 * t46707 - 0.14635184302277988245e0_f64 * t36058 - 0.5987120850931904282e-1_f64 * t46710 - 0.35922725105591425692e0_f64 * t25820 * t46575 * t333 - 0.5987120850931904282e-1_f64 * t46715 - 0.35922725105591425692e0_f64 * t4669 * t8975 * t1614 + 0.23948483403727617128e0_f64 * t5259 * t8975 * t1587 - 0.23948483403727617128e0_f64 * t5148 * t41059 * t570 - 0.35922725105591425692e0_f64 * t4669 * t40983 * t558 + 0.11974241701863808564e0_f64 * t305 * t45527 + 0.23948483403727617128e0_f64 * t5259 * t46582 * t321;
    t46734
}

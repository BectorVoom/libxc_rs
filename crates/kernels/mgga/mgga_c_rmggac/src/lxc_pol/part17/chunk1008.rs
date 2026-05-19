//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1008/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1008<F: Float>(t551: F, t8957: F, t1704: F, t664: F, t2367: F, t6376: F, t665: F, t1743: F, t2124: F, t45166: F, t5148: F, t1652: F, t27094: F, t27101: F, t305: F, t326: F, t333: F, t352: F, t41059: F, t41439: F, t46494: F, t46550: F, t4669: F, t5155: F, t5266: F, t558: F, t8975: F) -> (F, F, F, F, F, F, F) {
    let t46563 = t8957 * t551;
    let t46575 = t664 * t1704;
    let t46582 = t2367 * t551;
    let t46586 = t665 * t6376;
    let t46589 = t2124 * t1743;
    let t46592 = t664 * t1743;
    let t46599 = t5148 * t45166;
    let t46601 = F::cast_from(0.11974241701863808564e0_f64) * t305 * t46563 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t46494 * t333 - F::cast_from(0.11974241701863808564e1_f64) * t27094 * t46550 * t333 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t41059 * t558 - F::cast_from(0.23948483403727617128e0_f64) * t27101 * t46575 * t352 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t8975 * t1652 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t46582 * t352 + t41439 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t46586 - F::cast_from(0.59871208509319042821e-1_f64) * t326 * t46589 + F::cast_from(0.23948483403727617128e0_f64) * t5155 * t46592 * t333 + F::cast_from(0.11974241701863808564e0_f64) * t5266 * t46592 * t352 + F::cast_from(0.5987120850931904282e-1_f64) * t46599;
    (t46563, t46575, t46582, t46586, t46589, t46592, t46601)
}

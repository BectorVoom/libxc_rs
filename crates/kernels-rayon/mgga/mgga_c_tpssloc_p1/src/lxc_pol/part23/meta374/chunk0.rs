//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1175/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1175(t35577: f64, t1454: f64, t2585: f64, t1406: f64, t9238: f64, t4199: f64, t9919: f64, t9892: f64, t13123: f64, t9882: f64, t9888: f64, t9905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45496 = 1.0_f64 / t35577;
    let t45656 = t2585 * t1454;
    let t45844 = t1406 * t9238;
    let t46125 = t4199 * t9919;
    let t46130 = t4199 * t9892;
    let t46132 = t13123 * t9882;
    let t46134 = t13123 * t9888;
    let t46196 = t4199 * t9905;
    (t45496, t45656, t45844, t46125, t46130, t46132, t46134, t46196)
}

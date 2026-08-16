//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2482/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2482(t4199: f64, t9892: f64, t13123: f64, t9882: f64, t9888: f64, t118: f64, t2375: f64, t4095: f64, t9905: f64, t2517: f64, t3966: f64, t707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46130 = t4199 * t9892;
    let t46132 = t13123 * t9882;
    let t46134 = t13123 * t9888;
    let t46137 = t4095 * t118 * t2375;
    let t46196 = t4199 * t9905;
    let t46206 = t707 * t2517 * t3966;
    (t46130, t46132, t46134, t46137, t46196, t46206)
}

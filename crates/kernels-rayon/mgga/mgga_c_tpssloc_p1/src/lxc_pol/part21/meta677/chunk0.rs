//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2483/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2483(t4199: f64, t9494: f64, t13471: f64, t870: f64, t12945: f64, t2427: f64, t12858: f64, t2528: f64, t2371: f64, t4205: f64, t9909: f64, t13123: f64, t9885: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46208 = t4199 * t9494;
    let t46213 = t13471 * t870;
    let t46217 = t2427 * t12945;
    let t46234 = t12858 * t2528;
    let t46236 = t12858 * t2371;
    let t46244 = t4205 * t9909;
    let t46278 = t13123 * t9885;
    (t46208, t46213, t46217, t46234, t46236, t46244, t46278)
}

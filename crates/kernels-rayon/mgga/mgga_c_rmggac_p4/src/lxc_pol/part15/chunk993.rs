//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 993/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk993(t1652: f64, t2347: f64, t262: f64, t7788: f64, t45731: f64, t7785: f64, t558: f64, t8957: f64, t1734: f64, t2064: f64, t793: f64, t326: f64, t35862: f64, t35877: f64, t40976: f64, t41021: f64, t41029: f64, t41033: f64, t41037: f64, t41042: f64, t41057: f64) -> (f64, f64, f64, f64, f64) {
    let t46357 = t2347 * t1652;
    let t46358 = t262 * t46357;
    let t46359 = t7788 * t46358;
    let t46361 = t7785 * t45731;
    let t46365 = t8957 * t558;
    let t46369 = t2064 * t1734;
    let t46370 = t793 * t46369;
    let t46372 = 0.72732431077987577944e-1_f64 * t40976 + 0.66671395154821946449e-1_f64 * t41021 - 0.20001418546446583934e0_f64 * t41029 + 0.26668558061928778579e0_f64 * t41033 + 0.20455996240684006296e-1_f64 * t46359 + 0.81823984962736025184e-1_f64 * t46361 - 0.72732431077987577943e-1_f64 * t41037 - t41042 + 0.54549323308490683457e-1_f64 * t41057 - 0.11974241701863808564e0_f64 * t326 * t46365 - t35862 - 0.10000709273223291967e0_f64 * t35877 - 0.79828278012425390427e-1_f64 * t46370;
    (t46357, t46358, t46365, t46369, t46372)
}

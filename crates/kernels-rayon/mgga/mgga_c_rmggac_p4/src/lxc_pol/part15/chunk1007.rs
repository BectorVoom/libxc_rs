//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1007/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1007(t46437: f64, t5259: f64, t1734: f64, t664: f64, t25877: f64, t305: f64, t321: f64, t45769: f64, t46523: f64, t46527: f64, t46531: f64, t46535: f64, t46539: f64, t46543: f64, t46547: f64, t46550: f64, t46554: f64) -> (f64, f64) {
    let t46556 = t5259 * t46437;
    let t46558 = t664 * t1734;
    let t46562 = -0.13637330827122670864e0_f64 * t46523 - 0.27274661654245341728e-1_f64 * t46527 - 0.27274661654245341728e-1_f64 * t46531 - 0.20455996240684006297e-1_f64 * t46535 + 0.27274661654245341729e-1_f64 * t46539 + 0.20455996240684006297e-1_f64 * t46543 + 0.59871208509319042821e-1_f64 * t305 * t45769 + 0.59871208509319042821e-1_f64 * t305 * t46547 + 0.71845450211182851384e0_f64 * t25877 * t46550 * t321 - 0.8980681276397856423e-1_f64 * t46554 - 0.2993560425465952141e-1_f64 * t46556 + 0.11974241701863808564e0_f64 * t5259 * t46558 * t321;
    (t46558, t46562)
}

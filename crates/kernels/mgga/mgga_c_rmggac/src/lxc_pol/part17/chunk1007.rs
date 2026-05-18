//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1007/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1007<F: Float>(t46437: F, t5259: F, t1734: F, t664: F, t25877: F, t305: F, t321: F, t45769: F, t46523: F, t46527: F, t46531: F, t46535: F, t46539: F, t46543: F, t46547: F, t46550: F, t46554: F) -> (F, F) {
    let t46556 = t5259 * t46437;
    let t46558 = t664 * t1734;
    let t46562 = -F::new(0.13637330827122670864e0) * t46523 - F::new(0.27274661654245341728e-1) * t46527 - F::new(0.27274661654245341728e-1) * t46531 - F::new(0.20455996240684006297e-1) * t46535 + F::new(0.27274661654245341729e-1) * t46539 + F::new(0.20455996240684006297e-1) * t46543 + F::new(0.59871208509319042821e-1) * t305 * t45769 + F::new(0.59871208509319042821e-1) * t305 * t46547 + F::new(0.71845450211182851384e0) * t25877 * t46550 * t321 - F::new(0.8980681276397856423e-1) * t46554 - F::new(0.2993560425465952141e-1) * t46556 + F::new(0.11974241701863808564e0) * t5259 * t46558 * t321;
    (t46558, t46562)
}

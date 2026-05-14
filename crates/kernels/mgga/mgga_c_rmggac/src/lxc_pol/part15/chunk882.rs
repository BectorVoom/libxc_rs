//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 882/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk882<F: Float>(t35929: F, t46542: F, t5840: F, t665: F, t1737: F, t664: F, t46261: F, t5271: F, t46437: F, t5259: F, t1734: F, t25877: F, t305: F, t321: F, t45769: F, t46523: F, t46527: F, t46531: F, t46535: F, t46539: F) -> (F, F, F, F) {
    let t46543 = t35929 * t46542;
    let t46547 = t665 * t5840;
    let t46550 = t664 * t1737;
    let t46554 = t5271 * t46261;
    let t46556 = t5259 * t46437;
    let t46558 = t664 * t1734;
    let t46562 = -0.13637330827122670864e0 * t46523 - 0.27274661654245341728e-1 * t46527 - 0.27274661654245341728e-1 * t46531 - 0.20455996240684006297e-1 * t46535 + 0.27274661654245341729e-1 * t46539 + 0.20455996240684006297e-1 * t46543 + 0.59871208509319042821e-1 * t305 * t45769 + 0.59871208509319042821e-1 * t305 * t46547 + 0.71845450211182851384e0 * t25877 * t46550 * t321 - 0.8980681276397856423e-1 * t46554 - 0.2993560425465952141e-1 * t46556 + 0.11974241701863808564e0 * t5259 * t46558 * t321;
    (t46547, t46550, t46558, t46562)
}

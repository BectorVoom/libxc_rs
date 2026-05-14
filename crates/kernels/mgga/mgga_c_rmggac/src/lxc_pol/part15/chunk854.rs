//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 854/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk854<F: Float>(t118: F, t40804: F, t40807: F, t40832: F, t45626: F, t46047: F, t46050: F, t46056: F, t46059: F, t46062: F, t46064: F, t46066: F, t46069: F, t1652: F, t8800: F, t6376: F, t645: F) -> (F, F, F) {
    let t46071 = -0.39914139006212695214e-1 * t118 * t46047 + 0.11974241701863808564e0 * t118 * t46050 - t40804 - t40807 - 0.39914139006212695214e-1 * t118 * t45626 - 0.47896966807455234256e0 * t46056 - 0.79828278012425390427e-1 * t46059 + t40832 - 0.20455996240684006296e-1 * t46062 - 0.2993560425465952141e-1 * t46064 + 0.5987120850931904282e-1 * t46066 - 0.8980681276397856423e-1 * t46069;
    let t46072 = t8800 * t1652;
    let t46075 = t645 * t6376;
    (t46071, t46072, t46075)
}

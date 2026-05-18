//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 970/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk970<F: Float>(t2061: F, t9908: F, t15093: F, t9005: F, t1704: F, t325: F, t2057: F, t118: F, t40804: F, t40807: F, t40832: F, t45626: F, t46047: F, t46050: F, t46056: F, t46059: F, t46062: F) -> (F, F) {
    let t46064 = t9908 * t2061;
    let t46066 = t15093 * t9005;
    let t46068 = t1704 * t325;
    let t46069 = t46068 * t2057;
    let t46071 = -F::new(0.39914139006212695214e-1) * t118 * t46047 + F::new(0.11974241701863808564e0) * t118 * t46050 - t40804 - t40807 - F::new(0.39914139006212695214e-1) * t118 * t45626 - F::new(0.47896966807455234256e0) * t46056 - F::new(0.79828278012425390427e-1) * t46059 + t40832 - F::new(0.20455996240684006296e-1) * t46062 - F::new(0.2993560425465952141e-1) * t46064 + F::new(0.5987120850931904282e-1) * t46066 - F::new(0.8980681276397856423e-1) * t46069;
    (t46068, t46071)
}

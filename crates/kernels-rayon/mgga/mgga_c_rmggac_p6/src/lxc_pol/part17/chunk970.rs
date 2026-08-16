//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 970/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk970(t2061: f64, t9908: f64, t15093: f64, t9005: f64, t1704: f64, t325: f64, t2057: f64, t118: f64, t40804: f64, t40807: f64, t40832: f64, t45626: f64, t46047: f64, t46050: f64, t46056: f64, t46059: f64, t46062: f64) -> (f64, f64) {
    let t46064 = t9908 * t2061;
    let t46066 = t15093 * t9005;
    let t46068 = t1704 * t325;
    let t46069 = t46068 * t2057;
    let t46071 = -0.39914139006212695214e-1_f64 * t118 * t46047 + 0.11974241701863808564e0_f64 * t118 * t46050 - t40804 - t40807 - 0.39914139006212695214e-1_f64 * t118 * t45626 - 0.47896966807455234256e0_f64 * t46056 - 0.79828278012425390427e-1_f64 * t46059 + t40832 - 0.20455996240684006296e-1_f64 * t46062 - 0.2993560425465952141e-1_f64 * t46064 + 0.5987120850931904282e-1_f64 * t46066 - 0.8980681276397856423e-1_f64 * t46069;
    (t46068, t46071)
}

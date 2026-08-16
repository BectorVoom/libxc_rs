//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 980/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk980(t46128: f64, t793: f64, t41271: f64, t41298: f64, t41300: f64, t41303: f64, t41308: f64, t41315: f64, t41320: f64, t43566: f64, t43571: f64, t46189: f64, t46191: f64, t46193: f64, t46195: f64, t46197: f64, t46199: f64) -> f64 {
    let t46201 = t793 * t46128;
    let t46205 = -0.2419210303588817044e-3_f64 * t46189 + 0.28224120208536198848e-3_f64 * t46191 - 0.18183107769496894486e-1_f64 * t46193 - 0.60610359231656314955e-1_f64 * t46195 + t43566 - t43571 - 0.33190385262651453347e-3_f64 * t46197 + 0.14967802127329760705e-1_f64 * t46199 + 0.26609426004141796809e-1_f64 * t46201 - 0.20697688152926545822e-2_f64 * t41271 - t41298 - t41300 - t41303 + 0.72732431077987577944e-1_f64 * t41308 + t41315 - t41320;
    t46205
}

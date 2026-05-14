//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 861/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk861<F: Float>(t2115: F, t46129: F, t2118: F, t46177: F, t46181: F, t7633: F, t46185: F, t7641: F, t46116: F, t851: F, t46121: F, t797: F, t46128: F, t793: F, t41271: F, t41298: F, t41300: F, t41303: F, t41308: F, t41315: F, t41320: F, t43566: F, t43571: F) -> (F,) {
    let t46189 = t2115 * t46129;
    let t46191 = t2118 * t46177;
    let t46193 = t7633 * t46181;
    let t46195 = t7641 * t46185;
    let t46197 = t851 * t46116;
    let t46199 = t797 * t46121;
    let t46201 = t793 * t46128;
    let t46205 = -0.2419210303588817044e-3 * t46189 + 0.28224120208536198848e-3 * t46191 - 0.18183107769496894486e-1 * t46193 - 0.60610359231656314955e-1 * t46195 + t43566 - t43571 - 0.33190385262651453347e-3 * t46197 + 0.14967802127329760705e-1 * t46199 + 0.26609426004141796809e-1 * t46201 - 0.20697688152926545822e-2 * t41271 - t41298 - t41300 - t41303 + 0.72732431077987577944e-1 * t41308 + t41315 - t41320;
    (t46205,)
}

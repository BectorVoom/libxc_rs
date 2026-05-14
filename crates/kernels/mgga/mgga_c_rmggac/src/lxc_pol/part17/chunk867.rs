//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 867/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk867<F: Float>(t41176: F, t6387: F, t3814: F, t46184: F, t46121: F, t854: F, t6444: F, t9872: F, t46116: F, t793: F, t36174: F, t43622: F, t46266: F, t46268: F, t46270: F, t46272: F, t46274: F, t46276: F, t46279: F, t46281: F, t46283: F) -> (F,) {
    let t46285 = t41176 * t6387;
    let t46287 = t3814 * t46184;
    let t46289 = t854 * t46121;
    let t46291 = t6444 * t9872;
    let t46293 = t793 * t46116;
    let t46295 = -t36174 - 0.27879923620627220811e-2 * t46266 + 0.2993560425465952141e-1 * t46268 + 0.19914231157590872008e-2 * t46270 + 0.19914231157590872008e-2 * t46272 - 0.19957069503106347607e-1 * t46274 + 0.2993560425465952141e-1 * t46276 - 0.13276154105060581339e-2 * t46279 - 0.23948483403727617128e0 * t46281 + 0.15931384926072697606e-1 * t46283 - 0.27879923620627220811e-1 * t46285 + 0.15965655602485078085e0 * t46287 + 0.39828462315181744016e-3 * t46289 - 0.99785347515531738034e-2 * t46291 - 0.99785347515531738034e-2 * t46293 + t43622;
    (t46295,)
}

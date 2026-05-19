//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 988/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk988<F: Float>(t46116: F, t793: F, t36174: F, t43622: F, t46266: F, t46268: F, t46270: F, t46272: F, t46274: F, t46276: F, t46279: F, t46281: F, t46283: F, t46285: F, t46287: F, t46289: F, t46291: F) -> F {
    let t46293 = t793 * t46116;
    let t46295 = -t36174 - F::cast_from(0.27879923620627220811e-2_f64) * t46266 + F::cast_from(0.2993560425465952141e-1_f64) * t46268 + F::cast_from(0.19914231157590872008e-2_f64) * t46270 + F::cast_from(0.19914231157590872008e-2_f64) * t46272 - F::cast_from(0.19957069503106347607e-1_f64) * t46274 + F::cast_from(0.2993560425465952141e-1_f64) * t46276 - F::cast_from(0.13276154105060581339e-2_f64) * t46279 - F::cast_from(0.23948483403727617128e0_f64) * t46281 + F::cast_from(0.15931384926072697606e-1_f64) * t46283 - F::cast_from(0.27879923620627220811e-1_f64) * t46285 + F::cast_from(0.15965655602485078085e0_f64) * t46287 + F::cast_from(0.39828462315181744016e-3_f64) * t46289 - F::cast_from(0.99785347515531738034e-2_f64) * t46291 - F::cast_from(0.99785347515531738034e-2_f64) * t46293 + t43622;
    t46295
}

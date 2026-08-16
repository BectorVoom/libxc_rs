//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1052/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1052(t26283: f64, t26287: f64, t26291: f64, t30204: f64, t40719: f64, t40724: f64, t46333: f64, t46336: f64, t46339: f64, t46382: f64, t46400: f64, t46403: f64, t46406: f64, t47263: f64, t47265: f64, t47267: f64, t47269: f64, t47271: f64, t47275: f64, t47280: f64, t47287: f64, t47292: f64, t884: f64) -> f64 {
    let t47294 = -0.86737941314158990623e-4_f64 * t40719 + 0.71845450211182851384e0_f64 * t26287 * t46333 - 0.14369090042236570277e1_f64 * t26283 * t46336 - 0.71845450211182851384e0_f64 * t26291 * t46339 + 0.47896966807455234256e0_f64 * t30204 * t46400 - 0.71845450211182851384e0_f64 * t26291 * t46403 - 0.71845450211182851384e0_f64 * t40724 * t46406 + 0.17025839957319135759e-4_f64 * t47263 + 0.85129199786595678796e-5_f64 * t47265 + 0.3192344991997337955e-4_f64 * t47267 + 0.1064114997332445985e-4_f64 * t47269 - 0.1064114997332445985e-4_f64 * t47271 + 0.11971293719990017331e-4_f64 * t47275 - 0.17025839957319135759e-4_f64 * t47280 + 0.59871208509319042821e-1_f64 * t884 * t46382 + 0.25538759935978703639e-4_f64 * t47287 - 0.25538759935978703639e-4_f64 * t47292;
    t47294
}

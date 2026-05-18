//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1052/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1052<F: Float>(t26283: F, t26287: F, t26291: F, t30204: F, t40719: F, t40724: F, t46333: F, t46336: F, t46339: F, t46382: F, t46400: F, t46403: F, t46406: F, t47263: F, t47265: F, t47267: F, t47269: F, t47271: F, t47275: F, t47280: F, t47287: F, t47292: F, t884: F) -> F {
    let t47294 = -F::new(0.86737941314158990623e-4) * t40719 + F::new(0.71845450211182851384e0) * t26287 * t46333 - F::new(0.14369090042236570277e1) * t26283 * t46336 - F::new(0.71845450211182851384e0) * t26291 * t46339 + F::new(0.47896966807455234256e0) * t30204 * t46400 - F::new(0.71845450211182851384e0) * t26291 * t46403 - F::new(0.71845450211182851384e0) * t40724 * t46406 + F::new(0.17025839957319135759e-4) * t47263 + F::new(0.85129199786595678796e-5) * t47265 + F::new(0.3192344991997337955e-4) * t47267 + F::new(0.1064114997332445985e-4) * t47269 - F::new(0.1064114997332445985e-4) * t47271 + F::new(0.11971293719990017331e-4) * t47275 - F::new(0.17025839957319135759e-4) * t47280 + F::new(0.59871208509319042821e-1) * t884 * t46382 + F::new(0.25538759935978703639e-4) * t47287 - F::new(0.25538759935978703639e-4) * t47292;
    t47294
}

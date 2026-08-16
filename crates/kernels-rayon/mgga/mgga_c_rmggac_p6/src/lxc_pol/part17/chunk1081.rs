//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1081/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1081(t5055: f64, t9008: f64, t46526: f64, t7192: f64, t46530: f64, t34938: f64, t46534: f64, t34944: f64, t46538: f64, t41738: f64, t46542: f64, t4044: f64, t6400: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t47719 = t5055 * t9008;
    let t47721 = t7192 * t46526;
    let t47723 = t7192 * t46530;
    let t47725 = t34938 * t46534;
    let t47727 = t34944 * t46538;
    let t47729 = t41738 * t46542;
    let t47735 = t4044 * t645 * t6400;
    (t47719, t47721, t47723, t47725, t47727, t47729, t47735)
}

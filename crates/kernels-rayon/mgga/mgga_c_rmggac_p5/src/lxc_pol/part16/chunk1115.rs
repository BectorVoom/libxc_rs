//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1115/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1115(t37544: f64, t41363: f64, t46266: f64, t46268: f64, t46270: f64, t46272: f64, t46274: f64, t46276: f64, t46279: f64, t46281: f64, t46283: f64, t46285: f64, t46287: f64, t46289: f64, t46291: f64, t46293: f64) -> f64 {
    let t49143 = -t37544 - 0.55759847241254441624e-2_f64 * t46266 + 0.5987120850931904282e-1_f64 * t46268 + 0.39828462315181744017e-2_f64 * t46270 + 0.39828462315181744017e-2_f64 * t46272 - 0.39914139006212695214e-1_f64 * t46274 + 0.59871208509319042821e-1_f64 * t46276 - 0.26552308210121162678e-2_f64 * t46279 - 0.47896966807455234256e0_f64 * t46281 + 0.31862769852145395214e-1_f64 * t46283 - 0.55759847241254441624e-1_f64 * t46285 + 0.31931311204970156171e0_f64 * t46287 + 0.79656924630363488034e-3_f64 * t46289 - 0.19957069503106347607e-1_f64 * t46291 - 0.19957069503106347607e-1_f64 * t46293 + 0.1333427903096438929e0_f64 * t41363;
    t49143
}

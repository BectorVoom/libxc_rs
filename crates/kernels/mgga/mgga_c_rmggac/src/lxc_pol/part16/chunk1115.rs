//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1115/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1115<F: Float>(t37544: F, t41363: F, t46266: F, t46268: F, t46270: F, t46272: F, t46274: F, t46276: F, t46279: F, t46281: F, t46283: F, t46285: F, t46287: F, t46289: F, t46291: F, t46293: F) -> F {
    let t49143 = -t37544 - F::cast_from(0.55759847241254441624e-2_f64) * t46266 + F::cast_from(0.5987120850931904282e-1_f64) * t46268 + F::cast_from(0.39828462315181744017e-2_f64) * t46270 + F::cast_from(0.39828462315181744017e-2_f64) * t46272 - F::cast_from(0.39914139006212695214e-1_f64) * t46274 + F::cast_from(0.59871208509319042821e-1_f64) * t46276 - F::cast_from(0.26552308210121162678e-2_f64) * t46279 - F::cast_from(0.47896966807455234256e0_f64) * t46281 + F::cast_from(0.31862769852145395214e-1_f64) * t46283 - F::cast_from(0.55759847241254441624e-1_f64) * t46285 + F::cast_from(0.31931311204970156171e0_f64) * t46287 + F::cast_from(0.79656924630363488034e-3_f64) * t46289 - F::cast_from(0.19957069503106347607e-1_f64) * t46291 - F::cast_from(0.19957069503106347607e-1_f64) * t46293 + F::cast_from(0.1333427903096438929e0_f64) * t41363;
    t49143
}

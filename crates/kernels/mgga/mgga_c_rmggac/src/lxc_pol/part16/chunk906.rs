//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 906/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk906<F: Float>(t1956: F, t2046: F, t2050: F, t31: F, t1954: F, t2039: F, t270: F, t638: F, t5055: F, t9008: F, t46526: F, t7192: F, t46530: F, t34938: F, t46534: F, t34944: F, t46538: F) -> (F, F, F, F, F, F, F, F) {
    let t47706 = t2046 * t2050 * t1956 * t31;
    let t47710 = t638 * t2039 * t1954 * t270;
    let t47714 = t2046 * t2050 * t1954 * t31;
    let t47719 = t5055 * t9008;
    let t47721 = t7192 * t46526;
    let t47723 = t7192 * t46530;
    let t47725 = t34938 * t46534;
    let t47727 = t34944 * t46538;
    (t47706, t47710, t47714, t47719, t47721, t47723, t47725, t47727)
}

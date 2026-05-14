//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 848/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk848<F: Float>(t16043: F, t9964: F, t30344: F, t3351: F, t3352: F, t515: F, t17787: F, t9005: F, t10112: F, t325: F, t2057: F, t12970: F, t2061: F, t1737: F, t2064: F, t3814: F) -> (F, F, F, F, F, F, F) {
    let t46034 = t16043 * t9964;
    let t46038 = t3351 * t3352 * t515 * t30344;
    let t46040 = t17787 * t9005;
    let t46042 = t10112 * t325;
    let t46043 = t46042 * t2057;
    let t46045 = t12970 * t2061;
    let t46055 = t2064 * t1737;
    let t46056 = t3814 * t46055;
    (t46034, t46038, t46040, t46043, t46045, t46055, t46056)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1053/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1053<F: Float>(t47917: F, t7717: F, t3351: F, t498: F, t515: F, t6522: F, t7248: F, t26287: F, t46394: F, t46385: F, t30204: F, t46388: F) -> (F, F, F, F, F) {
    let t47918 = t7717 * t47917;
    let t47923 = t3351 * t7248 * t515 * t6522 * t498;
    let t47931 = t26287 * t46394;
    let t47933 = t26287 * t46385;
    let t47935 = t30204 * t46388;
    (t47918, t47923, t47931, t47933, t47935)
}

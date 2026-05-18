//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1002/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1002<F: Float>(t1864: F, t236: F, t498: F, t7231: F, t8517: F, t321: F, t3352: F, t1971: F, t333: F, t511: F, t352: F, t515: F) -> (F, F, F, F) {
    let t46933 = t8517 * t7231 * t236 * t1864 * t498;
    let t46938 = t8517 * t3352 * t236 * t1864 * t321;
    let t46943 = t8517 * t1971 * t511 * t1864 * t333;
    let t46948 = t8517 * t1971 * t515 * t1864 * t352;
    (t46933, t46938, t46943, t46948)
}

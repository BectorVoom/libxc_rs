//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 915/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk915<F: Float>(t26287: F, t46394: F, t46385: F, t30204: F, t46388: F, t1502: F, t16503: F, t16504: F, t552: F, t10078: F, t34761: F, t34962: F, t8420: F, t1756: F, t3351: F, t498: F, t515: F, t7231: F) -> (F, F, F, F, F, F, F) {
    let t47931 = t26287 * t46394;
    let t47933 = t26287 * t46385;
    let t47935 = t30204 * t46388;
    let t47946 = t16503 * t16504 * t552 * t1502;
    let t47948 = t34761 * t10078;
    let t47952 = t16503 * t34962 * t552 * t8420;
    let t47957 = t3351 * t7231 * t515 * t1756 * t498;
    (t47931, t47933, t47935, t47946, t47948, t47952, t47957)
}

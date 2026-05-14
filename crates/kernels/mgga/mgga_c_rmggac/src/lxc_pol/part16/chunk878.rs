//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 878/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk878<F: Float>(t352: F, t515: F, t7230: F, t7231: F, t9988: F, t118: F, t2001: F, t2281: F, t615: F, t7717: F, t1818: F, t1970: F, t209: F, t236: F, t476: F, t9210: F) -> (F, F, F) {
    let t46958 = t7230 * t7231 * t515 * t9988 * t352;
    let t46962 = t2001 * t118 * t2281 * t615;
    let t46963 = t7717 * t46962;
    let t46969 = t1970 * t9210 * t236 * t1818 * t476 * t209;
    (t46958, t46963, t46969)
}

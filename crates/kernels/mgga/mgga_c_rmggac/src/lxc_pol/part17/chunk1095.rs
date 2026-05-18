//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1095/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1095<F: Float>(t10100: F, t3352: F, t352: F, t515: F, t7230: F, t1986: F, t305: F, t552: F, t615: F, t7717: F, t3351: F, t498: F, t6522: F, t7248: F) -> (F, F, F) {
    let t47913 = t7230 * t3352 * t515 * t10100 * t352;
    let t47917 = t1986 * t305 * t552 * t615;
    let t47918 = t7717 * t47917;
    let t47923 = t3351 * t7248 * t515 * t6522 * t498;
    (t47913, t47918, t47923)
}

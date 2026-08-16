//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 871/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk871<F: Float>(t236: F, t321: F, t3351: F, t35155: F, t9182: F, t333: F, t511: F, t7248: F, t352: F, t515: F, t2001: F, t305: F, t498: F, t552: F) -> (F, F, F, F) {
    let t39157 = t3351 * t35155 * t236 * t9182 * t321;
    let t39162 = t3351 * t7248 * t511 * t9182 * t333;
    let t39167 = t3351 * t7248 * t515 * t9182 * t352;
    let t39171 = t2001 * t305 * t552 * t498;
    (t39157, t39162, t39167, t39171)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 857/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk857<F: Float>(t3351: F, t352: F, t515: F, t7248: F, t9182: F, t2001: F, t305: F, t498: F, t552: F, t7720: F, t236: F, t495: F, t7230: F) -> (F, F, F) {
    let t39167 = t3351 * t7248 * t515 * t9182 * t352;
    let t39171 = t2001 * t305 * t552 * t498;
    let t39172 = t7720 * t39171;
    let t39177 = t7230 * t7248 * t236 * t9182 * t495;
    (t39167, t39172, t39177)
}

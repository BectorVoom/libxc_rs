//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 790/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk790<F: Float>(t1971: F, t333: F, t511: F, t6108: F, t7365: F, t352: F, t515: F, t1707: F, t3351: F, t498: F, t7248: F, t1525: F, t209: F, t236: F, t605: F, t7453: F) -> (F, F, F, F) {
    let t44610 = t7365 * t1971 * t511 * t6108 * t333;
    let t44615 = t7365 * t1971 * t515 * t6108 * t352;
    let t44620 = t3351 * t7248 * t511 * t1707 * t498;
    let t44627 = t7453 * t1971 * t236 * t1525 * t605 * t209;
    (t44610, t44615, t44620, t44627)
}

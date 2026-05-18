//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 868/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk868<F: Float>(t236: F, t498: F, t6108: F, t7231: F, t7365: F, t321: F, t3352: F, t1971: F, t333: F, t511: F, t352: F, t515: F) -> (F, F, F, F) {
    let t44600 = t7365 * t7231 * t236 * t6108 * t498;
    let t44605 = t7365 * t3352 * t236 * t6108 * t321;
    let t44610 = t7365 * t1971 * t511 * t6108 * t333;
    let t44615 = t7365 * t1971 * t515 * t6108 * t352;
    (t44600, t44605, t44610, t44615)
}

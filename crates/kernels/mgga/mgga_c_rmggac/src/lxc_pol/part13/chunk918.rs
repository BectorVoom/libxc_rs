//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 918/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk918<F: Float>(t118: F, t2001: F, t2318: F, t498: F, t7717: F, t1462: F, t1971: F, t333: F, t511: F, t8517: F, t352: F, t515: F) -> (F, F, F) {
    let t40231 = t2001 * t118 * t2318 * t498;
    let t40232 = t7717 * t40231;
    let t40237 = t8517 * t1971 * t511 * t1462 * t333;
    let t40242 = t8517 * t1971 * t515 * t1462 * t352;
    (t40232, t40237, t40242)
}

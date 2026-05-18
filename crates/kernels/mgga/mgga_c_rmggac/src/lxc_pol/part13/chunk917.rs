//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 917/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk917<F: Float>(t2010: F, t2415: F, t4018: F, t8342: F, t938: F, t333: F, t511: F, t7230: F, t7231: F, t8666: F, t352: F, t515: F) -> (F, F, F, F) {
    let t40214 = t2010 * t2415 * t4018;
    let t40217 = t2010 * t8342 * t938;
    let t40222 = t7230 * t7231 * t511 * t8666 * t333;
    let t40227 = t7230 * t7231 * t515 * t8666 * t352;
    (t40214, t40217, t40222, t40227)
}

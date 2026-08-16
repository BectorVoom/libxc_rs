//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 832/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk832<F: Float>(t1528: F, t236: F, t321: F, t3351: F, t7248: F, t333: F, t511: F, t7231: F, t352: F, t515: F, t7720: F, t8582: F) -> (F, F, F, F) {
    let t38594 = t3351 * t7248 * t236 * t1528 * t321;
    let t38599 = t3351 * t7231 * t511 * t1528 * t333;
    let t38604 = t3351 * t7231 * t515 * t1528 * t352;
    let t38606 = t7720 * t8582;
    (t38594, t38599, t38604, t38606)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 918/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk918<F: Float>(t2283: F, t38472: F, t2286: F, t38638: F, t10084: F, t16156: F, t3351: F, t3352: F, t44713: F, t515: F, t7720: F, t9795: F) -> (F, F, F, F, F) {
    let t45329 = t38472 * t2283;
    let t45331 = t38638 * t2286;
    let t45333 = t16156 * t10084;
    let t45337 = t3351 * t3352 * t515 * t44713;
    let t45339 = t7720 * t9795;
    (t45329, t45331, t45333, t45337, t45339)
}

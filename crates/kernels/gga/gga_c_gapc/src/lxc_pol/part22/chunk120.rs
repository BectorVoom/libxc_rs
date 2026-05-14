//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 120/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk120<F: Float>(t11: F, t1: F, t351: F, t21: F, t84: F, t352: F, t354: F, t30: F, t347: F) -> (F, F, F, F, F, F, F) {
    let t356 = f64::sqrt(t11);
    let t357 = t356 * t1;
    let t358 = t357 * t351;
    let t360 = t21 * t84;
    let t362 = -0.632975e0 * t352 - 0.29896666666666666667e0 * t354 - 0.1023875e0 * t358 - 0.82156666666666666667e-1 * t360;
    let t363 = 1.0 / t30;
    let t364 = t362 * t363;
    let t366 = 1.0 * t347 * t364;
    (t357, t358, t360, t362, t363, t364, t366)
}

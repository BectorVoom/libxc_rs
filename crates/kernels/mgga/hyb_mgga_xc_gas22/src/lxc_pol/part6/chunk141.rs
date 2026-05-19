//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 141/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk141<F: Float>(t419: F, t423: F, t407: F, t328: F, t409: F, t198: F, t212: F, t398: F, t401: F, t405: F, t408: F, t410: F, t414: F, t415: F) -> (F, F, F, F, F) {
    let t424 = t419 * t423;
    let t427 = t407 * t407;
    let t428 = t328 * t427;
    let t429 = t409 * t409;
    let t430 = F::new(1.0) / t429;
    let t431 = t428 * t430;
    let t436 = F::cast_from(0.46914023462026644e0_f64) * t398 * t198 * t401 + t405 * t212 + t408 * t410 + F::cast_from(0.10661445329398457901e-1_f64) * t415 * t424 + F::cast_from(0.10661445329398457901e-1_f64) * t431 * t414 * t419 * t423;
    (t428, t429, t430, t431, t436)
}

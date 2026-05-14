//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 168/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk168<F: Float>(t406: F, t415: F, t241: F, t391: F, t393: F, t402: F, rho1: F) -> (F, F, F, F, F) {
    let t416 = t406 * t415;
    let t419 = t241 * (-0.3109e-1 * t393 * t402 + t391 - 0.19751789702565206229e-1 * t416);
    let t421 = 0.19751789702565206229e-1 * t241 * t416;
    let t422 = rho1 * rho1;
    let t423 = pow_1_3(rho1);
    let t424 = t423 * t423;
    let t426 = 1.0 / t424 / t422;
    (t419, t421, t422, t423, t426)
}

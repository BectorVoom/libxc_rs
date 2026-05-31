//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 156/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk156<F: Float>(t406: F, t415: F, t241: F, t391: F, t393: F, t402: F, rho1: F) -> (F, F, F) {
    let t416 = t406 * t415;
    let t419 = t241 * (-F::cast_from(0.3109e-1_f64) * t393 * t402 + t391 - F::cast_from(0.19751789702565206229e-1_f64) * t416);
    let t421 = F::cast_from(0.19751789702565206229e-1_f64) * t241 * t416;
    let t422 = rho1 * rho1;
    (t419, t421, t422)
}

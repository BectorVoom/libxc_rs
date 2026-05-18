//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 125/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk125<F: Float>(t68: F, t69: F, t62: F, t1: F, t348: F, t65: F, t352: F, t354: F, t14: F, t351: F) -> (F, F, F, F, F) {
    let t391 = F::new(1.0) / t69 / t68;
    let t392 = t62 * t391;
    let t394 = t348 * t65 * t1;
    let t399 = -F::new(0.66066666666666666667e-2) * t352 - F::new(0.41275e-2) * t354;
    let t402 = -t394 * t351 / F::new(12.0) + t14 * t399 / F::new(2.0);
    (t391, t392, t394, t399, t402)
}

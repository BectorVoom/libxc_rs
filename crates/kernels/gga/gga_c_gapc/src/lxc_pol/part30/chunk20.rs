//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 20/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk20<F: Float>(t46: F, t55: F, t44: F, t22: F, t7: F, t6: F) -> (F, F, F, F) {
    let t56 = t46 * t55;
    let t58 = 0.19751789702565206229e-1 * t44 * t56;
    let t60 = 1.0 / t22 / t7;
    let t61 = t6 * t60;
    (t56, t58, t60, t61)
}

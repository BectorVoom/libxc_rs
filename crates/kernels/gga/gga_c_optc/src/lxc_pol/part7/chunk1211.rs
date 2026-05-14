//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1211/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1211<F: Float>(t26278: F, t26289: F, t26300: F, t26306: F, t26309: F, t26419: F, t26425: F, t26428: F, t26430: F, t26433: F, t26435: F, t26443: F, t26446: F, t26448: F, t2972: F, t393: F) -> (F, F) {
    let t26657 = 0.62517e0 * t26419 - 0.6618234375e1 * t26425 - 0.94674375e0 * t26428 + 0.1262325e1 * t26430 + 0.158837625e2 * t26433 - 0.705945e1 * t26435 - 0.34431666666666666667e1 * t26278 + 0.123954e2 * t26289 - 0.185931e2 * t26300 - 0.41318e1 * t26306 + 0.13772666666666666667e1 * t26309 + 0.125034e1 * t26443 - 0.375102e1 * t26446 + 0.6311625e0 * t26448;
    let t26663 = t2972 * t2972;
    let t26665 = t393 / t26663;
    (t26657, t26665)
}

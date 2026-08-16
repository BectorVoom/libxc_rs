//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 895/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk895<F: Float>(t22634: F, t2684: F, t9438: F, t2033: F, t2365: F, t40586: F, t28924: F, t6111: F, t12656: F, t22665: F, t7427: F, t29285: F) -> (F, F, F, F, F) {
    let t41448 = t2684 * t9438 * t22634;
    let t41451 = t2033 * t2365 * t40586;
    let t41454 = t6111 * t2365 * t28924;
    let t41457 = t7427 * t22665 * t12656;
    let t41460 = t6111 * t2365 * t29285;
    (t41448, t41451, t41454, t41457, t41460)
}

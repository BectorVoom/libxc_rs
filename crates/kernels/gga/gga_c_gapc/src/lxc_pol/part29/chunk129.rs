//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 129/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk129<F: Float>(t413: F, t8: F, t6: F, t93: F, t408: F, t411: F, t414: F, t88: F, t392: F, t402: F, t405: F, t70: F, t73: F, t99: F) -> (F, F, F, F, F) {
    let t417 = 1.0 / t8 / t413;
    let t418 = t6 * t417;
    let t419 = t93 * t418;
    let t421 = 0.59778596625315888114e-2 * t88 - 0.17565e-2 * t408 + 0.39625e-3 * t411 - 0.1294884726949076719e-4 * t414 + 0.1260328125e-5 * t419;
    let t423 = -0.11713266981940447749e-2 * t88 * t70 - 0.23426533963880895498e-2 * t392 * t402 - t405 * t99 - t73 * t421;
    (t417, t418, t419, t421, t423)
}

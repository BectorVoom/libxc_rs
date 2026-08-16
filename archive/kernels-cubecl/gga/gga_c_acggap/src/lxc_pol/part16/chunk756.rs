//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 756/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk756<F: Float>(t1165: F, t1427: F, t604: F, t8463: F, t1181: F, t1416: F, t7575: F, t1345: F, t7351: F, t1350: F, t7426: F, t1164: F, t2325: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8465 = t1165 * t604 * t1427;
    let t8466 = t8463 * t8465;
    let t8469 = t1181 * t604 * t1416;
    let t8470 = t7575 * t8469;
    let t8473 = t1165 * t7351 * t1345;
    let t8474 = t7575 * t8473;
    let t8476 = t604 * t1350;
    let t8477 = t1181 * t8476;
    let t8478 = t7426 * t8477;
    let t8480 = t1164 * t2325;
    (t8465, t8466, t8469, t8470, t8473, t8474, t8476, t8477, t8478, t8480)
}

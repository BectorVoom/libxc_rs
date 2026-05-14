//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 971/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk971<F: Float>(t3088: F, t407: F, t545: F, t864: F, t119: F, t181: F, t469: F, t5392: F, t11509: F, t11510: F, t11511: F, t11512: F, t11514: F, t11515: F, t2618: F, t3986: F, t3990: F, t3993: F, t5441: F, t5475: F) -> (F, F, F, F) {
    let t19262 = t3088 * t545 * t864 * t407;
    let t19278 = t119 * t181;
    let t19289 = t5392 * t469;
    let t19345 = -12.0 * t3986 - t11509 + 4.0 * t3990 - 12.0 * t5441 + t11510 + 0.65061487801810439052e-1 * t3993 + t11511 + t11512 + 0.65061487801810439052e-1 * t2618 - t11514 - t11515 + 0.39503346997227602814e-1 * t5475;
    (t19262, t19278, t19289, t19345)
}

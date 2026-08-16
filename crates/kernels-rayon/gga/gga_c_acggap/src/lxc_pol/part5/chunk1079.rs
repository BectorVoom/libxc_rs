//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1079/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1079(t1647: f64, t3101: f64, t381: f64, t3088: f64, t407: f64, t545: f64, t864: f64, t119: f64, t181: f64, t469: f64, t5392: f64, t11509: f64, t11510: f64, t11511: f64, t11512: f64, t11514: f64, t11515: f64, t2618: f64, t3986: f64, t3990: f64, t3993: f64, t5441: f64, t5475: f64) -> (f64, f64, f64, f64, f64) {
    let t19255 = t381 * t1647 * t3101;
    let t19262 = t3088 * t545 * t864 * t407;
    let t19278 = t119 * t181;
    let t19289 = t5392 * t469;
    let t19345 = -12.0_f64 * t3986 - t11509 + 4.0_f64 * t3990 - 12.0_f64 * t5441 + t11510 + 0.65061487801810439052e-1_f64 * t3993 + t11511 + t11512 + 0.65061487801810439052e-1_f64 * t2618 - t11514 - t11515 + 0.39503346997227602814e-1_f64 * t5475;
    (t19255, t19262, t19278, t19289, t19345)
}

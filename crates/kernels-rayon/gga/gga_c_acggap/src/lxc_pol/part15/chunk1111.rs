//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1111/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1111(t1980: f64, t38795: f64, t7476: f64, t2001: f64, t5950: f64, t1861: f64, t7605: f64, t1181: f64, t604: f64, t6192: f64, t7426: f64, t5876: f64, t7575: f64) -> (f64, f64, f64, f64, f64) {
    let t39189 = t1980 * t7476 * t38795;
    let t39192 = t2001 * t5950;
    let t39194 = t7605 * t1861;
    let t39203 = t7426 * t1181 * t604 * t6192;
    let t39209 = t7575 * t1181 * t604 * t5876;
    (t39189, t39192, t39194, t39203, t39209)
}

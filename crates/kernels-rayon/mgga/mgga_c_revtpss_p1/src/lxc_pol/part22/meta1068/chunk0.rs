//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3819/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3819(t47009: f64, t47011: f64, t22461: f64, t4147: f64, t48267: f64, t48269: f64, t47016: f64, t1448: f64, t22287: f64, t39786: f64, t39791: f64, t39795: f64, t4139: f64, t4140: f64, t5536: f64, t5541: f64, t6816: f64, t9547: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73402 = 0.24415263074675393405e-3_f64 * t47009;
    let t73403 = 0.11393789434848516922e-2_f64 * t47011;
    let t73407 = t22461 * t4147;
    let t73411 = 0.97661052298701573622e-3_f64 * t48267;
    let t73412 = 0.10389515463408878255e3_f64 * t48269;
    let t73416 = 480.0_f64 * t47016;
    let t73417 = -2.0_f64 * t1448 * t5541 * t73407 + 12.0_f64 * t22287 * t4140 * t5536 + 3.0_f64 * t4139 * t6816 * t9547 - t39786 - t39791 - t39795 + t73402 - t73403 + t73411 - t73412 - t73416;
    (t73402, t73403, t73411, t73412, t73416, t73417)
}

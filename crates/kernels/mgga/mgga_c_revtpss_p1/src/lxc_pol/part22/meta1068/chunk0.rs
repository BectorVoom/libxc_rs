//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3819/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3819<F: Float>(t47009: F, t47011: F, t22461: F, t4147: F, t48267: F, t48269: F, t47016: F, t1448: F, t22287: F, t39786: F, t39791: F, t39795: F, t4139: F, t4140: F, t5536: F, t5541: F, t6816: F, t9547: F) -> (F, F, F, F, F, F) {
    let t73402 = F::cast_from(0.24415263074675393405e-3_f64) * t47009;
    let t73403 = F::cast_from(0.11393789434848516922e-2_f64) * t47011;
    let t73407 = t22461 * t4147;
    let t73411 = F::cast_from(0.97661052298701573622e-3_f64) * t48267;
    let t73412 = F::cast_from(0.10389515463408878255e3_f64) * t48269;
    let t73416 = F::cast_from(480.0_f64) * t47016;
    let t73417 = -F::cast_from(2.0_f64) * t1448 * t5541 * t73407 + F::cast_from(12.0_f64) * t22287 * t4140 * t5536 + F::cast_from(3.0_f64) * t4139 * t6816 * t9547 - t39786 - t39791 - t39795 + t73402 - t73403 + t73411 - t73412 - t73416;
    (t73402, t73403, t73411, t73412, t73416, t73417)
}

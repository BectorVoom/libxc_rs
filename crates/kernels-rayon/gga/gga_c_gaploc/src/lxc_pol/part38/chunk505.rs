//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 505/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk505(t6576: f64, t9544: f64, t2334: f64, t2465: f64, t2464: f64, t587: f64, t3177: f64, t6985: f64, t2487: f64, t589: f64, t2365: f64, t6510: f64) -> (f64, f64, f64, f64, f64) {
    let t9545 = t6576 * t9544;
    let t9546 = 0.38342925953920749676e0_f64 * t9545;
    let t9547 = t2465 * t2334;
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9550 = 0.85206502119823888169e-1_f64 * t9549;
    let t9552 = t6985 * t3177;
    let t9553 = t2487 * t9552;
    let t9554 = 0.51123901271894332901e0_f64 * t9553;
    let t9555 = t589 * t3177;
    let t9556 = t587 * t9555;
    let t9557 = 0.51123901271894332901e0_f64 * t9556;
    let t9558 = t2365 * t6510;
    (t9546, t9550, t9554, t9557, t9558)
}

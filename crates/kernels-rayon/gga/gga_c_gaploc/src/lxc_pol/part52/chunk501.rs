//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 501/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk501(t6576: f64, t9544: f64, t2334: f64, t2465: f64, t2464: f64, t587: f64, t3177: f64, t6985: f64, t2487: f64, t589: f64, t2365: f64, t6510: f64) -> (f64, f64, f64, f64, f64) {
    let t9545 = t6576 * t9544;
    let t9547 = t2465 * t2334;
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9552 = t6985 * t3177;
    let t9553 = t2487 * t9552;
    let t9555 = t589 * t3177;
    let t9556 = t587 * t9555;
    let t9558 = t2365 * t6510;
    (t9545, t9549, t9553, t9556, t9558)
}

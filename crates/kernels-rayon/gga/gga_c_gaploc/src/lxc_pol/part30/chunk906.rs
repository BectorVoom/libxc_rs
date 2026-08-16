//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 906/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk906(t2464: f64, t9547: f64, t587: f64, t3177: f64, t6985: f64, t2487: f64, t589: f64, t2365: f64, t6510: f64, t4391: f64, t544: f64, t6851: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
    let t9560 = 0.59584149919750711116e-1_f64 * t4391 * t9558;
    let t9561 = t544 * t6851;
    (t9548, t9550, t9552, t9554, t9555, t9557, t9558, t9560, t9561)
}

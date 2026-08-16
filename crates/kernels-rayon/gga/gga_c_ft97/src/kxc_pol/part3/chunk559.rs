//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 559/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk559(t79: f64, t1742: f64, t4417: f64, t420: f64, t419: f64, t423: f64, t4431: f64, t1731: f64, t3086: f64, t4481: f64, t409: f64, t64: f64, t1599: f64, t1624: f64, t372: f64, t4442: f64, t4446: f64, t4450: f64, t4468: f64, t4471: f64, t4476: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t80 = 0.1e-59_f64 < t79;
    let t4483 = t1742 * t4417;
    let t4484 = t420 * t4483;
    let t4485 = t419 * t4484;
    let t4487 = t423 * t4431;
    let t4488 = t420 * t4487;
    let t4489 = t419 * t4488;
    let t4491 = -t1731 + 0.42562405586419753086e-2_f64 * t3086 + 0.85124811172839506173e-2_f64 * t4481 - 0.12768721675925925926e-1_f64 * t4485 + 0.6384360837962962963e-2_f64 * t4489;
    let t4492 = t409 * t4491;
    let t4493 = t64 * t4492;
    let t4495 = piecewise3(t80, 0.67598802253579164263e-4_f64 * t4442 * t1599 + 0.23254900946437792e-1_f64 * t1624 * t4446 + 0.23254900946437792e-2_f64 * t372 * t4450 - 0.11627450473218896e-1_f64 * t372 * t4468 + 0.19365723406274399941e-3_f64 * t372 * t4471 + 2.0_f64 * t4476 - t4493, 0.0_f64);
    (t4483, t4484, t4485, t4487, t4488, t4489, t4491, t4493, t4495)
}

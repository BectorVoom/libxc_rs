//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 671/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk671(t4593: f64, t961: f64, t1323: f64, t3282: f64, t1008: f64, t1429: f64, t1150: f64, t3324: f64, t3326: f64, t3330: f64, t3344: f64, t3349: f64, t335: f64, t3358: f64, t3364: f64, t3368: f64, t3373: f64, t3376: f64, t3380: f64, t367: f64, t4571: f64, t4574: f64, t4579: f64, t4583: f64, t4587: f64, t4590: f64) -> (f64, f64, f64, f64) {
    let t4594 = t4593 * t961;
    let t4597 = t3282 * t1323;
    let t4603 = t1008 * t1429;
    let t4613 = t1150 * t4571 / 8.0_f64 + t335 * t4574 / 24.0_f64 + t1150 * t4579 / 8.0_f64 + t335 * t4583 / 24.0_f64 + t335 * t4587 / 48.0_f64 + t367 * t4590 / 24.0_f64 + t335 * t4594 / 24.0_f64 + t335 * t4597 / 24.0_f64 + 0.10003937560882938627e-2_f64 * t3324 - 0.21437009059034868486e-3_f64 * t3326 - 0.20007875121765877254e-2_f64 * t3330 + 0.85748036236139473944e-2_f64 * t4603 + 0.42874018118069736972e-3_f64 * t3344 + 0.85748036236139473944e-3_f64 * t3349 - 0.34299214494455789578e-2_f64 * t3358 + 0.34299214494455789578e-2_f64 * t3364 - 0.34299214494455789578e-2_f64 * t3368 - 0.40015750243531754508e-2_f64 * t3373 + 0.85748036236139473944e-3_f64 * t3376 + 0.17149607247227894789e-2_f64 * t3380;
    (t4594, t4597, t4603, t4613)
}

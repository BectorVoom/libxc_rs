//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 610/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk610(t1017: f64, t336: f64, t4643: f64, t1140: f64, t1511: f64, t1137: f64, t1494: f64, t1498: f64, t1150: f64, t335: f64, t3383: f64, t3385: f64, t3394: f64, t3410: f64, t3412: f64, t3428: f64, t3432: f64, t3446: f64, t3449: f64, t3454: f64, t367: f64, t4627: f64, t4629: f64, t4632: f64, t4635: f64, t4637: f64, t4640: f64) -> (f64, f64) {
    let t4645 = t336 * t4643 * t1017;
    let t4649 = 7.0_f64 / 144.0_f64 * t1140 * t1511;
    let t4651 = 7.0_f64 / 72.0_f64 * t1137 * t1494;
    let t4653 = 7.0_f64 / 72.0_f64 * t1137 * t1498;
    let t4656 = -0.85748036236139473944e-3_f64 * t3383 + 0.85748036236139473944e-3_f64 * t3385 + 0.17149607247227894789e-2_f64 * t3394 + 0.40015750243531754508e-2_f64 * t3410 - 0.40015750243531754508e-2_f64 * t3412 + 0.42874018118069736972e-3_f64 * t3428 - 0.80031500487063509016e-2_f64 * t3432 + 0.21437009059034868486e-3_f64 * t3446 - t4627 + t4629 - t335 * t4632 / 24.0_f64 - 35.0_f64 / 216.0_f64 * t4635 - 35.0_f64 / 432.0_f64 * t4637 + t1150 * t4640 / 16.0_f64 + t367 * t4645 / 48.0_f64 + t4649 + t4651 + t4653 + 0.10003937560882938627e-2_f64 * t3449 - 0.85748036236139473944e-3_f64 * t3454;
    (t4645, t4656)
}

//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 710/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk710(t3462: f64, t38: f64, t1289: f64, t2033: f64, t3431: f64, t608: f64, t2040: f64, t612: f64, t581: f64, t77: f64, t1291: f64, t1307: f64, t1314: f64, t3427: f64, t3433: f64, t3436: f64, t3441: f64, t583: f64, t603: f64, t616: f64, t71: f64, t85: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3463 = t38 * t3462;
    let t3472 = t2033 * t1289;
    let t3475 = t608 * t3431;
    let t3477 = t2040 * t1289;
    let t3480 = t612 * t3431;
    let t3482 = 28.0_f64 / 9.0_f64 * t3472 * t581 - 4.0_f64 / 3.0_f64 * t3475 + 28.0_f64 / 9.0_f64 * t3477 * t581 + 4.0_f64 / 3.0_f64 * t3480;
    let t3483 = t77 * t3482;
    let t3486 = -t3427 * t85 / 12.0_f64 - t3433 * t85 / 12.0_f64 - t3436 * t85 / 12.0_f64 - t1291 * t616 / 12.0_f64 - t3441 * t85 / 12.0_f64 + t3463 * t85 / 24.0_f64 + t1307 * t616 / 24.0_f64 - t583 * t1314 / 12.0_f64 + t603 * t1314 / 24.0_f64 + t71 * t3483 / 24.0_f64;
    (t3463, t3472, t3477, t3482, t3483, t3486)
}

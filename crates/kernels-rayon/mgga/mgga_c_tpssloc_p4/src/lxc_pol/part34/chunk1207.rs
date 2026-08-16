//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1207/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1207(t107056: f64, t107214: f64, t20029: f64, t20044: f64, t20613: f64, t27009: f64, t6461: f64, t7194: f64, t7925: f64, t7937: f64, t84423: f64, t97529: f64, t97537: f64, t97548: f64, t97571: f64) -> f64 {
    let t107731 = 0.46058153871750340221e0_f64 * t97529 - 0.3289868133696452873e-1_f64 * t107056 + t84423 + 12.0_f64 * t20029 * t7925 - 0.49348022005446793095e-1_f64 * t107214 + 0.23029076935875170111e0_f64 * t97537 - 0.23029076935875170111e0_f64 * t97548 - 3.0_f64 * t20044 * t7937 + 6.0_f64 * t7194 * t20613 - 3.0_f64 * t27009 * t6461 - 0.49348022005446793095e-1_f64 * t97571;
    t107731
}

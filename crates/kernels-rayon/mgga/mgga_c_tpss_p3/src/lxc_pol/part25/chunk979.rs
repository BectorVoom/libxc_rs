//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 979/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk979(t1329: f64, t2: f64, t555: f64, t2091: f64, t4665: f64, t636: f64, t13515: f64, t108: f64, t105: f64, t13181: f64, t13202: f64, t1327: f64, t13501: f64, t13505: f64, t13511: f64, t13516: f64, t13526: f64, t3525: f64, t3529: f64, t4650: f64, t4653: f64, t4656: f64, t631: f64, t637: f64, t97: f64) -> f64 {
    let t13529 = t1329 * t2;
    let t13530 = t13529 * t555;
    let t13533 = t2091 * t4665;
    let t13534 = t13533 * t636;
    let t13537 = -t13515;
    let t13538 = t108 * t13537;
    let t13541 = -50.0_f64 / 27.0_f64 * t631 * t4650 - 10.0_f64 / 27.0_f64 * t97 * t13501 + 20.0_f64 / 9.0_f64 * t13181 * t13505 - 25.0_f64 / 9.0_f64 * t631 * t4653 + 10.0_f64 / 9.0_f64 * t97 * t13511 + 5.0_f64 / 3.0_f64 * t97 * t13516 + 200.0_f64 / 27.0_f64 * t4656 * t637 - 100.0_f64 / 27.0_f64 * t1327 * t3525 + 50.0_f64 / 9.0_f64 * t1327 * t3529 - 10.0_f64 / 27.0_f64 * t105 * t13526 - 20.0_f64 / 9.0_f64 * t13202 * t13530 + 10.0_f64 / 9.0_f64 * t105 * t13534 + 5.0_f64 / 3.0_f64 * t105 * t13538;
    t13541
}

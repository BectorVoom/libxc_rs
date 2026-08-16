//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1381/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1381(t114: f64, t61871: f64, t61874: f64, t61876: f64, t63006: f64, t65447: f64, t65450: f64, t65453: f64, t65455: f64, t67531: f64, t67532: f64, t67533: f64, t116: f64, t20287: f64) -> (f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t67537 = -t63006 - 44.0_f64 / 9.0_f64 * t61871 - 4.0_f64 / 3.0_f64 * t61874 + 2.0_f64 / 3.0_f64 * t61876 - t67531 - t67532 + t67533 - 3.0_f64 / 2.0_f64 * t65447 + t65450 + t65453 / 2.0_f64 - t65455 / 4.0_f64;
    let t67538 = piecewise3(t115, 0.0_f64, t67537);
    let t67541 = t20287 * t116;
    (t67538, t67541)
}

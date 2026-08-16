//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 813/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk813(t4273: f64, t580: f64, t1509: f64, t2357: f64, t661: f64, t108: f64, t2: f64, t105: f64, t1505: f64, t1507: f64, t4270: f64, t656: f64, t662: f64, t97: f64) -> (f64, f64, f64, f64) {
    let t4274 = t4273 * t580;
    let t4279 = t2357 * t1509;
    let t4280 = t4279 * t661;
    let t4283 = t108 * t2;
    let t4284 = t4283 * t580;
    let t4287 = -25.0_f64 / 9.0_f64 * t656 * t1505 + 10.0_f64 / 9.0_f64 * t97 * t4270 + 5.0_f64 / 3.0_f64 * t97 * t4274 - 25.0_f64 / 9.0_f64 * t1507 * t662 + 10.0_f64 / 9.0_f64 * t105 * t4280 - 5.0_f64 / 3.0_f64 * t105 * t4284;
    (t4274, t4279, t4283, t4287)
}

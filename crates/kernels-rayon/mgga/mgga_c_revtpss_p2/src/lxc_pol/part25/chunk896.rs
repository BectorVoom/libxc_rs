//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 896/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk896(t760: f64, t9419: f64, t2516: f64, t2523: f64, t9387: f64, t2496: f64, t189: f64, t606: f64, t2258: f64, t4401: f64, t9372: f64, t37: f64, t716: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10592 = 0.10389515463408878255e3_f64 * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10594 = 0.17544670867903938621e1_f64 * t10593;
    let t10596 = 0.5848223622634646207e0_f64 * t760 * t9387;
    let t10597 = t2523 * t2496;
    let t10598 = 0.51947577317044391276e2_f64 * t10597;
    let t10599 = t189 * t606;
    let t10600 = t10599 * t2258;
    let t10602 = 36.0_f64 * t4401 * t10600;
    let t10604 = 0.10254018858216406658e4_f64 * t760 * t9372;
    let t10605 = t37 * t716;
    (t10592, t10594, t10596, t10598, t10602, t10604, t10605)
}

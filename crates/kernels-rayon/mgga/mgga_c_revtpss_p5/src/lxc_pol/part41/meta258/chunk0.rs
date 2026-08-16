//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 990/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk990(t185: f64, t2494: f64, t9367: f64, t9368: f64, t1340: f64, t2516: f64, t4038: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64) -> (f64, f64, f64, f64, f64) {
    let t9371 = 1.0_f64 / t2494 / t185;
    let t9372 = t9367 * t9368 * t9371;
    let t9374 = 0.10254018858216406658e4_f64 * t1340 * t9372;
    let t9375 = t4038 * t2516;
    let t9385 = -0.34523333333333333333e1_f64 * t9283 + 0.23015555555555555556e1_f64 * t9286 - 0.26851481481481481482e1_f64 * t9289 - 0.93932222222222222223e0_f64 * t9292 + 0.73355e-1_f64 * t9296 - 0.14671e0_f64 * t9298 - 0.17116166666666666667e0_f64 * t9300 - 0.36793333333333333333e0_f64 * t9303;
    (t9371, t9372, t9374, t9375, t9385)
}

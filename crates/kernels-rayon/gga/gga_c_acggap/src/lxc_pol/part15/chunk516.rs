//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 516/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk516(t3114: f64, t352: f64, t1059: f64, t1068: f64, t1072: f64, t301: f64, t21: f64, t5: f64, t137: f64, t167: f64, t130: f64, t985: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3115 = t352 * t3114;
    let t3124 = t1068 * t1059;
    let t3125 = t1072 * t301;
    let t3126 = t21 * t5;
    let t3127 = t3125 * t3126;
    let t3128 = t3124 * t3127;
    let t3132 = t167 * t137;
    let t3140 = t130 * t985;
    (t3115, t3124, t3125, t3126, t3128, t3132, t3140)
}

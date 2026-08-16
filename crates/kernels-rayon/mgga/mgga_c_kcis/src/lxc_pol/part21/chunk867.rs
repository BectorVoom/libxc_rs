//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 867/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk867(t1021: f64, t13346: f64, t1020: f64, t1121: f64, t167: f64, t3203: f64, t3202: f64, t13172: f64, t13145: f64, t13332: f64, t13337: f64, t13340: f64, t13344: f64, t2836: f64, t9557: f64, t9559: f64, t9563: f64, t9572: f64) -> (f64, f64, f64, f64) {
    let t13347 = t1021 * t13346;
    let t13348 = t1020 * t13347;
    let t13353 = t167 * t1121;
    let t13354 = t3203 * t13353;
    let t13355 = t3202 * t13354;
    let t13356 = t13172 * t13355;
    let t13359 = -0.22109259259259259258e-2_f64 * t13332 + 0.890445125e-2_f64 * t2836 * t13145 - 0.33163888888888888888e-2_f64 * t13337 - 0.11054629629629629629e-2_f64 * t13340 + 0.88437037037037037034e-2_f64 * t13344 + 0.1621345679012345679e-1_f64 * t13348 - 0.22109259259259259258e-2_f64 * t9557 - 0.58958024691358024689e-2_f64 * t9559 - 0.73697530864197530861e-3_f64 * t9563 - 0.66327777777777777776e-2_f64 * t13356 + 0.22109259259259259258e-2_f64 * t9572;
    (t13348, t13354, t13356, t13359)
}

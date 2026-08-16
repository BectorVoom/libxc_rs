//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1052/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1052(t13293: f64, t3200: f64, t1804: f64, t2845: f64, t3210: f64, t4554: f64, t13238: f64, t13243: f64, t13247: f64, t13252: f64, t13254: f64, t13258: f64, t13263: f64, t13268: f64, t13271: f64, t13275: f64, t13278: f64, t13282: f64, t13286: f64, t13290: f64, t9524: f64, t9529: f64, t9536: f64, t9552: f64) -> (f64, f64, f64) {
    let t13294 = t3200 * t13293;
    let t13296 = t1804 * t2845;
    let t13297 = t3210 * t13296;
    let t13298 = t4554 * t13297;
    let t13300 = -0.33163888888888888888e-2_f64 * t9524 - 0.88437037037037037034e-2_f64 * t13238 + t13243 + 0.11054629629629629629e-2_f64 * t9529 - 0.3684876543209876543e-3_f64 * t9536 + 0.88437037037037037034e-2_f64 * t13247 + 0.44218518518518518517e-2_f64 * t13252 - 0.33163888888888888888e-2_f64 * t13254 - 0.33163888888888888888e-2_f64 * t13258 + 0.55273148148148148147e-3_f64 * t13263 + 0.33163888888888888888e-2_f64 * t13268 + t13271 - 0.73697530864197530861e-3_f64 * t9552 + 0.13265555555555555555e-1_f64 * t13275 - t13278 - 0.16581944444444444444e-2_f64 * t13282 - 0.16581944444444444444e-2_f64 * t13286 - 0.27636574074074074073e-2_f64 * t13290 + 0.11054629629629629629e-2_f64 * t13294 + 0.18424382716049382715e-2_f64 * t13298;
    (t13294, t13298, t13300)
}

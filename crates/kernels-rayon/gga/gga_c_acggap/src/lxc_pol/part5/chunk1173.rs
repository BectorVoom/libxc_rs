//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1173/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1173(t1137: f64, t6290: f64, t1140: f64, t6294: f64, t1131: f64, t1150: f64, t1313: f64, t1524: f64, t1567: f64, t1884: f64, t3282: f64, t335: f64, t4099: f64, t4582: f64, t4586: f64, t4593: f64, t513: f64, t5235: f64, t5906: f64, t6300: f64, t6304: f64, t6388: f64, t960: f64) -> f64 {
    let t21230 = t1137 * t6290;
    let t21232 = t1140 * t6294;
    let t21257 = -t1150 * t960 * t1884 * t1131 / 16.0_f64 + t1150 * t960 * t1313 * t4099 / 8.0_f64 + t335 * t4593 * t4586 / 24.0_f64 - 7.0_f64 / 72.0_f64 * t21230 - 7.0_f64 / 72.0_f64 * t21232 + t335 * t4593 * t4582 / 12.0_f64 + t1150 * t3282 * t6388 / 8.0_f64 + t335 * t3282 * t5906 / 24.0_f64 + t335 * t3282 * t6300 / 12.0_f64 + t335 * t3282 * t6304 / 12.0_f64 + t335 * t960 * t5235 * t513 / 24.0_f64 + t335 * t960 * t1567 * t1524 / 12.0_f64;
    t21257
}

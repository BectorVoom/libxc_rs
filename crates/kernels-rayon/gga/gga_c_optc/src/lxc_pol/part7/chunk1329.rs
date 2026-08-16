//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1329/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1329(t26314: f64, t26319: f64, t26339: f64, t26343: f64, t26363: f64, t26365: f64, t26367: f64, t26369: f64, t26372: f64, t26376: f64, t26379: f64, t26382: f64, t26385: f64, t26388: f64) -> f64 {
    let t26523 = -0.89459259259259259259e0_f64 * t26339 - 0.301925e0_f64 * t26343 - 0.5519e0_f64 * t26363 - 0.18396666666666666667e0_f64 * t26365 + 0.22076e0_f64 * t26367 + 0.98115555555555555555e-1_f64 * t26369 + 0.40256666666666666668e0_f64 * t26314 + 0.11038e1_f64 * t26372 - 0.8585111111111111111e-1_f64 * t26376 - 0.82785e-1_f64 * t26379 - 0.22076e0_f64 * t26382 + 0.66228e0_f64 * t26385 - 0.11038e0_f64 * t26388 + 0.24154e1_f64 * t26319;
    t26523
}

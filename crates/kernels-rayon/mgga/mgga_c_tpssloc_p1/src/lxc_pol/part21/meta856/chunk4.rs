//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3100/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3100(t51043: f64, t51051: f64, t51053: f64, t63355: f64, t63359: f64, t63361: f64, t63365: f64, t63370: f64, t63374: f64, t63380: f64, t63382: f64, t63384: f64, t63388: f64, t63392: f64, t63396: f64) -> f64 {
    let t64229 = -0.83356000000000000002e0_f64 * t51043 - 0.15436296296296296297e0_f64 * t51051 - 0.55570666666666666668e0_f64 * t51053 + 0.516475e0_f64 * t63355 - 0.68863333333333333334e0_f64 * t63359 + 0.45908888888888888889e0_f64 * t63361 + 0.20659e1_f64 * t63365 - 0.20659e1_f64 * t63370 + 0.57386111111111111112e0_f64 * t63374 + 0.68863333333333333334e1_f64 * t63380 + 0.45908888888888888889e0_f64 * t63382 + 0.13772666666666666666e1_f64 * t63384 - 0.20659e1_f64 * t63388 - 0.123954e2_f64 * t63392 - 0.68863333333333333334e0_f64 * t63396;
    t64229
}

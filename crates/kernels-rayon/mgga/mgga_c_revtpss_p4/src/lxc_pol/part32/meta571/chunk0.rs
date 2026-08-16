//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1895/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1895(t101218: f64, t2047: f64, t28154: f64, t95296: f64, t28147: f64, t95319: f64, t28150: f64, t7348: f64, t25162: f64, t116: f64, t28651: f64, t2106: f64, t47672: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101938 = t2047 * t101218;
    let t101955 = 160.0_f64 / 9.0_f64 * t28154 * t95296;
    let t101969 = 160.0_f64 / 3.0_f64 * t95319 * t28147;
    let t101970 = t7348 * t28150;
    let t101972 = 160.0_f64 / 9.0_f64 * t25162 * t101970;
    let t102019 = t28651 * t116;
    let t102070 = t2106 * t47672;
    (t101938, t101955, t101969, t101970, t101972, t102019, t102070)
}

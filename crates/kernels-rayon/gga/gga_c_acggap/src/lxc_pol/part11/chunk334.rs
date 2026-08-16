//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 334/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk334(t43: f64, t50: f64, t702: f64, t705: f64, t474: f64, t817: f64, t292: f64, t34: f64, t234: f64, t821: f64, t478: f64, t829: f64, t296: f64, t238: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1279 = 4.0_f64 * t702;
    let t1280 = 0.18311447306006545054e-3_f64 * t705;
    let t1281 = t817 * t474;
    let t1284 = t292 * t34;
    let t1288 = piecewise3(t44, 0.0_f64, -2.0_f64 / 9.0_f64 * t1281 * t234 + 4.0_f64 / 3.0_f64 * t1284 * t821);
    let t1289 = t829 * t478;
    let t1292 = t296 * t34;
    let t1296 = piecewise3(t51, 0.0_f64, -2.0_f64 / 9.0_f64 * t1289 * t238 - 4.0_f64 / 3.0_f64 * t1292 * t821);
    (t1279, t1280, t1281, t1284, t1288, t1289, t1292, t1296)
}

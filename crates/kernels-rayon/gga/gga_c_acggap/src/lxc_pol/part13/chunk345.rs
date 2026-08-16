//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 345/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk345(t43: f64, t50: f64, t474: f64, t886: f64, t34: f64, t47: f64, t234: f64, t821: f64, t478: f64, t893: f64, t52: f64, t238: f64, t59: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1361 = t886 * t474;
    let t1364 = t47 * t34;
    let t1368 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t1361 * t234 + 8.0_f64 / 3.0_f64 * t1364 * t821);
    let t1369 = t893 * t478;
    let t1372 = t52 * t34;
    let t1376 = piecewise3(t51, 0.0_f64, 4.0_f64 / 9.0_f64 * t1369 * t238 - 8.0_f64 / 3.0_f64 * t1372 * t821);
    let t1378 = (t1368 + t1376) * t59;
    (t1361, t1364, t1369, t1372, t1378)
}

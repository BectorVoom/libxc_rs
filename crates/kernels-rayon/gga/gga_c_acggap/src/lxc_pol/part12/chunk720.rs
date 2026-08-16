//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 720/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk720(t1092: f64, t2001: f64, t1098: f64, t2118: f64, t957: f64, t1089: f64, t368: f64, t7554: f64, t7553: f64, t2037: f64, t7309: f64, t1966: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7663 = t2001 * t1092;
    let t7665 = t2001 * t1098;
    let t7667 = t2118 * t957;
    let t7670 = t1089 * t368 * t7554;
    let t7671 = t7553 * t7670;
    let t7673 = t7309 * t2037;
    let t7676 = t381 * t1966;
    (t7663, t7665, t7667, t7670, t7671, t7673, t7676)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1083/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1083(t14852: f64, t14854: f64, t14856: f64, t11591: f64, t11597: f64, t11566: f64, t11570: f64, t11574: f64, t11578: f64, t11582: f64, t11586: f64, t11596: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19396 = 0.11393789434848516923e-2_f64 * t14852;
    let t19397 = 0.10389515463408878255e3_f64 * t14854;
    let t19398 = 0.70178683471615754484e1_f64 * t14856;
    let t19399 = 0.11393789434848516922e-2_f64 * t11591;
    let t19400 = 0.10389515463408878255e3_f64 * t11597;
    let t19401 = -t19396 - t19397 + t19398 + t11566 + t11570 - t11574 + t11578 - t11582 - t11586 - t19399 + t11596 - t19400;
    (t19396, t19397, t19398, t19399, t19400, t19401)
}

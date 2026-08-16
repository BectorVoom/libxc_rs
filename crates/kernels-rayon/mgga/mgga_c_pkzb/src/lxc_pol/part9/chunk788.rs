//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 788/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk788(t276: f64, t5589: f64, t275: f64, t4784: f64, t2057: f64, t739: f64, t2045: f64, t735: f64, t291: f64, t3: f64, t197: f64, t290: f64, t297: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5591 = 5.0_f64 / 1296.0_f64 * t276 * t5589;
    let t5592 = t4784 * t275;
    let t5595 = t2057 * t739;
    let t5597 = t735 * t2045;
    let t5599 = t291 * t291;
    let t5601 = 1.0_f64 / t3 / t5599;
    let t5604 = t290 * t197 * t5601 * t297;
    (t5591, t5592, t5595, t5597, t5601, t5604)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 235/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk235(t298: f64, t891: f64, t181: f64, t282: f64, t6: f64, t481: f64, t311: f64, t315: f64, t435: f64, t122: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t892 = t298 * t891;
    let t893 = t181 * t892;
    let t896 = t282 * t6;
    let t897 = t896 * t481;
    let t898 = t311 * t897;
    let t899 = t435 * t315;
    let t902 = t282 * t122;
    let t903 = t902 * t188;
    let t904 = t311 * t903;
    (t892, t893, t896, t897, t898, t899, t902, t903, t904)
}

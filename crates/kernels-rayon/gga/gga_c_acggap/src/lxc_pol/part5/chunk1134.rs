//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1134/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1134(t406: f64, t495: f64, t1454: f64, t322: f64, t13287: f64, t13293: f64, t525: f64, t13298: f64, t176: f64, t5730: f64, t8401: f64, t13299: f64, t17173: f64, t5605: f64, t8790: f64) -> (f64, f64, f64, f64, f64) {
    let t20305 = t495 * t406;
    let t20311 = t1454 * t322;
    let t20314 = t13293 * t13287 * t525 * t20311;
    let t20323 = t13298 * t176 * t8401 * t5730;
    let t20328 = t17173 * t13299 * t8790 * t5605 * t322;
    (t20305, t20311, t20314, t20323, t20328)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 938/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk938(t17361: f64, t241: f64, t11671: f64, t14885: f64, t14887: f64, t14889: f64, t17338: f64, t17342: f64, t17346: f64, t17350: f64, t17354: f64, t17358: f64, t8662: f64) -> (f64, f64) {
    let t17363 = 0.19751789702565206229e-1_f64 * t241 * t17361;
    let t17380 = -t8662 - 4.0_f64 / 9.0_f64 * t11671 + 2.0_f64 / 9.0_f64 * t14885 - 2.0_f64 / 3.0_f64 * t14887 + t14889 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t17338 + 4.0_f64 / 3.0_f64 * t17342 - 2.0_f64 / 3.0_f64 * t17346 - 2.0_f64 * t17350 + 2.0_f64 * t17354 - t17358 / 3.0_f64;
    (t17363, t17380)
}

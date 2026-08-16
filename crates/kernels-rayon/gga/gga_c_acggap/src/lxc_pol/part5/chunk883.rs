//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 883/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk883(t13039: f64, t409: f64, t3206: f64, t334: f64, t339: f64, t1159: f64, t3054: f64, t1162: f64, t3453: f64, t3370: f64, t3401: f64, t1170: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13040 = t13039 * t409;
    let t13064 = t3206 * t334;
    let t13065 = t13064 * t339;
    let t13079 = t3054 * t1159;
    let t13080 = t13079 * t1162;
    let t13081 = t13080 * t3453;
    let t13083 = t3370 * t3401;
    let t13084 = t1170 * t13083;
    (t13040, t13064, t13065, t13079, t13080, t13081, t13083, t13084)
}

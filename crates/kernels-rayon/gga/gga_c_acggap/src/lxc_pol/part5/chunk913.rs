//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 913/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk913(t1096: f64, t1165: f64, t13232: f64, t3361: f64, t1037: f64, t1090: f64, t1181: f64, t12991: f64, t1195: f64, t3237: f64, t1200: f64, t1205: f64) -> (f64, f64, f64, f64, f64) {
    let t13923 = t3361 * t1165 * t13232 * t1096;
    let t13927 = t12991 * t1181 * t1037 * t1090;
    let t13929 = t3237 * t1195;
    let t13934 = t3237 * t1200;
    let t13936 = t3237 * t1205;
    (t13923, t13927, t13929, t13934, t13936)
}

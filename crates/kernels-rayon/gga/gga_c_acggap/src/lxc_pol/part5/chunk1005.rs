//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1005/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1005(t1181: f64, t3391: f64, t3529: f64, t4643: f64, t3759: f64, t10098: f64, t3402: f64, t4469: f64, t13860: f64, t4925: f64, t1413: f64, t3476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16893 = t3391 * t1181 * t4643 * t3529;
    let t16897 = t3391 * t1181 * t4643 * t3759;
    let t16899 = t10098 * t3402;
    let t16900 = t16899 * t4469;
    let t16902 = t13860 * t4925;
    let t16911 = t3476 * t1413;
    (t16893, t16897, t16899, t16900, t16902, t16911)
}

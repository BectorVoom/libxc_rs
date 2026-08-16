//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 554/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk554(t1039: f64, t3216: f64, t105: f64, t166: f64, t1: f64, t383: f64, t980: f64) -> (f64, f64, f64, f64) {
    let t3218 = 0.60023625365297631762e-2_f64 * t3216 * t1039;
    let t3220 = 1.0_f64 / t166 / t105;
    let t3221 = t3220 * t1;
    let t3228 = t980 * t383;
    (t3218, t3220, t3221, t3228)
}

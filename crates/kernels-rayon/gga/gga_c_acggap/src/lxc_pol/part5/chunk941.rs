//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 941/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk941(t1210: f64, t862: f64, t865: f64, t186: f64, t3873: f64, t150: f64, t1222: f64, t3892: f64, t323: f64, t3242: f64, t441: f64, t3101: f64, t316: f64, t449: f64, t463: f64) -> (f64, f64, f64, f64, f64) {
    let t14648 = t862 * t1210 * t865;
    let t14651 = 1.0_f64 / t3873 / t186;
    let t14652 = t150 * t14651;
    let t14671 = t3892 * t1222;
    let t14674 = t3242 * t441 * t323;
    let t14678 = t316 * t449 * t3101 * t463;
    (t14648, t14652, t14671, t14674, t14678)
}

//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 565/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk565(t1140: f64, t1507: f64, t145: f64, t1487: f64, t3570: f64, t500: f64, t3573: f64, t515: f64, t435: f64, t506: f64) -> (f64, f64, f64, f64, f64) {
    let t4629 = 7.0_f64 / 144.0_f64 * t1140 * t1507;
    let t4630 = t1487 * t145;
    let t4635 = t3570 * t500;
    let t4637 = t3573 * t515;
    let t4643 = t506 * t435;
    (t4629, t4630, t4635, t4637, t4643)
}

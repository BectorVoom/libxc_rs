//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 532/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk532(t1963: f64, t22: f64, t161: f64, t151: f64, t177: f64, t334: f64, t986: f64, t339: f64, t366: f64, t374: f64, t3106: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3558 = 1.0_f64 / t22 / t1963;
    let t3559 = t161 * t3558;
    let t3562 = 0.37792653007779990369e-1_f64 * t151 * t3559 * t177;
    let t3570 = t986 * t334;
    let t3571 = t3570 * t339;
    let t3573 = t986 * t366;
    let t3574 = t3573 * t374;
    let t3579 = 0.10866666666666666667e1_f64 * t3106;
    let t3580 = 0.978e0_f64 * t3109;
    (t3558, t3562, t3570, t3571, t3573, t3574, t3579, t3580)
}

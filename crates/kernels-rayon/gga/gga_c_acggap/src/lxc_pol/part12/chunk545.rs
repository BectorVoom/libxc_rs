//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 545/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk545(t334: f64, t986: f64, t339: f64, t366: f64, t374: f64, t1137: f64, t1145: f64, t3106: f64, t3109: f64, t3141: f64, t3160: f64, t19: f64, t2066: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3570 = t986 * t334;
    let t3571 = t3570 * t339;
    let t3573 = t986 * t366;
    let t3574 = t3573 * t374;
    let t3576 = t1137 * t1145;
    let t3579 = 0.10866666666666666667e1_f64 * t3106;
    let t3580 = 0.978e0_f64 * t3109;
    let t3588 = 0.38033333333333333333e1_f64 * t3141;
    let t3592 = 0.12225e1_f64 * t3160;
    let t3615 = t2066 * t19;
    (t3570, t3571, t3573, t3574, t3576, t3579, t3580, t3588, t3592, t3615)
}

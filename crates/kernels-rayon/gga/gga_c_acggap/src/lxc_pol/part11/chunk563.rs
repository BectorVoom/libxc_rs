//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 563/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk563(t316: f64, t3912: f64, t3044: f64, t317: f64, t863: f64, t463: f64, t864: f64, t449: f64, t180: f64, t3035: f64, t3037: f64, t3242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3914 = 0.65854491829355115987e0_f64 * t316 * t3912;
    let t3915 = t317 * t3044;
    let t3917 = 0.39512695097613069591e1_f64 * t863 * t3915;
    let t3918 = t864 * t463;
    let t3919 = t449 * t3918;
    let t3920 = t863 * t3919;
    let t3922 = t3035 * t180;
    let t3923 = t317 * t3037;
    let t3925 = 0.39512695097613069591e1_f64 * t3922 * t3923;
    let t3930 = t3242 * t180;
    (t3914, t3915, t3917, t3919, t3920, t3923, t3925, t3930)
}

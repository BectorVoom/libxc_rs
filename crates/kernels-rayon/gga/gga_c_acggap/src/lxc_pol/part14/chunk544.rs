//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 544/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk544(t3918: f64, t449: f64, t863: f64, t180: f64, t3035: f64, t3037: f64, t317: f64, t3242: f64, t323: f64, t868: f64, t880: f64, t3054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3919 = t449 * t3918;
    let t3920 = t863 * t3919;
    let t3922 = t3035 * t180;
    let t3923 = t317 * t3037;
    let t3925 = 0.39512695097613069591e1_f64 * t3922 * t3923;
    let t3930 = t3242 * t180;
    let t3932 = 0.19756347548806534796e1_f64 * t3930 * t323;
    let t3935 = t868 * t880;
    let t3937 = t3054 * t180;
    (t3919, t3920, t3923, t3925, t3932, t3935, t3937)
}

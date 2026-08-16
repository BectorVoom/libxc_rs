//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 613/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk613(t1220: f64, t3908: f64, t316: f64, t3101: f64, t317: f64, t3044: f64, t863: f64, t463: f64, t864: f64, t449: f64, t180: f64, t3035: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3909 = t1220 * t3908;
    let t3910 = t316 * t3909;
    let t3912 = t317 * t3101;
    let t3914 = 0.65854491829355115987e0_f64 * t316 * t3912;
    let t3915 = t317 * t3044;
    let t3917 = 0.39512695097613069591e1_f64 * t863 * t3915;
    let t3918 = t864 * t463;
    let t3919 = t449 * t3918;
    let t3920 = t863 * t3919;
    let t3922 = t3035 * t180;
    (t3909, t3910, t3912, t3914, t3915, t3917, t3918, t3919, t3920, t3922)
}

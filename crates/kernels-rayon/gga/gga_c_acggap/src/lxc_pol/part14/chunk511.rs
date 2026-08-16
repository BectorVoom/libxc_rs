//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 511/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk511(t3044: f64, t452: f64, t1035: f64, t309: f64, t861: f64, t150: f64, t1233: f64, t1240: f64, t864: f64, t180: f64, t879: f64, t407: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3045 = t452 * t3044;
    let t3047 = 0.39512695097613069591e1_f64 * t1035 * t3045;
    let t3054 = t309 * t861;
    let t3055 = t3054 * t150;
    let t3057 = 0.39512695097613069591e1_f64 * t3055 * t1233;
    let t3058 = t1240 * t864;
    let t3059 = t1035 * t3058;
    let t3065 = t180 * t879;
    let t3066 = t3065 * t407;
    (t3047, t3054, t3055, t3057, t3059, t3066)
}

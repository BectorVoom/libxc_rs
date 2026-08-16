//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 535/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk535(t1233: f64, t3055: f64, t1240: f64, t864: f64, t1035: f64, t322: f64, t407: f64, t441: f64, t1160: f64, t180: f64, t879: f64, t1236: f64, t930: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3057 = 0.39512695097613069591e1_f64 * t3055 * t1233;
    let t3058 = t1240 * t864;
    let t3059 = t1035 * t3058;
    let t3062 = t441 * t322 * t407;
    let t3063 = t1160 * t3062;
    let t3065 = t180 * t879;
    let t3066 = t3065 * t407;
    let t3067 = t1160 * t3066;
    let t3070 = t1236 * t930;
    (t3057, t3058, t3059, t3062, t3063, t3065, t3066, t3067, t3070)
}

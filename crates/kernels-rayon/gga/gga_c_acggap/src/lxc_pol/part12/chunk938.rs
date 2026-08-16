//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 938/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk938(t7310: f64, t7487: f64, t2082: f64, t30044: f64, t2087: f64, t7610: f64, t1092: f64, t7605: f64, t381: f64, t7779: f64, t2100: f64, t1096: f64, t1983: f64, t7380: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31279 = t7310 * t7487;
    let t31283 = t30044 * t2082;
    let t31285 = t7610 * t2087;
    let t31287 = t7605 * t1092;
    let t31289 = t381 * t7779;
    let t31290 = t31289 * t2100;
    let t31293 = t7380 * t1983 * t1096;
    (t31279, t31283, t31285, t31287, t31289, t31290, t31293)
}

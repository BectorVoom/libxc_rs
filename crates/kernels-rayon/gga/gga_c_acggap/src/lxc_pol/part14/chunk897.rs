//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 897/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk897(t30907: f64, t30589: f64, t7548: f64, t2109: f64, t7630: f64, t2113: f64, t30546: f64, t7499: f64, t2450: f64, t7432: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30908 = 0.12004725073059526352e-1_f64 * t30907;
    let t30920 = t30589 * t7548;
    let t30921 = 0.41930789719472202756e-2_f64 * t30920;
    let t30924 = t7630 * t2109;
    let t30926 = t7630 * t2113;
    let t30928 = t30546 * t7499;
    let t30934 = t2450 * t7432;
    (t30908, t30921, t30924, t30926, t30928, t30934)
}

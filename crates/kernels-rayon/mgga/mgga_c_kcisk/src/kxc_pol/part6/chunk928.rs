//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 928/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk928(t29439: f64, t29490: f64, t716: f64, t736: f64, t2576: f64, t9082: f64, t2567: f64, t9035: f64, t734: f64, t17936: f64, t9047: f64, t28950: f64, t719: f64, sigma2: f64) -> (f64, f64, f64, f64, f64) {
    let t29491 = t29439 + t29490;
    let t29492 = t29491 * t716;
    let t29493 = t29492 * sigma2;
    let t29494 = t29493 * t736;
    let t29496 = t2576 * t9082;
    let t29498 = t2567 * t9035;
    let t29499 = t734 * t29498;
    let t29501 = t17936 * t9047;
    let t29503 = t719 * t28950;
    (t29494, t29496, t29499, t29501, t29503)
}

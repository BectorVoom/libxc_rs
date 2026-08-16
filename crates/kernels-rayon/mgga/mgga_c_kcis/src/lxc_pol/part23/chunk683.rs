//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 683/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk683(t1555: f64, t7940: f64, t2253: f64, t4184: f64, t4189: f64, t1528: f64, t573: f64, t1532: f64, t491: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7941 = t7940 * t1555;
    let t7942 = t4184 * t2253;
    let t7943 = t2253 * t1555;
    let t7945 = 2.0_f64 * t4189 * t7943;
    let t7946 = t1528 * t573;
    let t7948 = t1532 * t491;
    (t7941, t7942, t7943, t7945, t7946, t7948)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1023/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1023(t3803: f64, t6631: f64, t3257: f64, t3028: f64, t5: f64, t337: f64, t2121: f64, t3116: f64, t3854: f64, t2170: f64, t2171: f64, t2168: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11450 = t3803 * t6631;
    let t11451 = t3257 * t11450;
    let t11454 = t5 * t3028;
    let t11455 = t337 * t11454;
    let t11456 = t2121 * t11455;
    let t11458 = t3116 * t11456 / 96.0_f64;
    let t11459 = t5 * t3854;
    let t11461 = t2170 * t11459 * t2171;
    let t11463 = t2168 * t11461 / 48.0_f64;
    (t11450, t11451, t11455, t11458, t11459, t11461, t11463)
}

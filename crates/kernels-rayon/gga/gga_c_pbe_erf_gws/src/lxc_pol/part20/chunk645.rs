//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 645/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk645(t1640: f64, t3522: f64, t639: f64, t3346: f64, t591: f64, t590: f64, t587: f64, t1664: f64, t3342: f64, t1661: f64, t1017: f64, t2635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3523 = t1640 * t3522;
    let t3525 = 4.0_f64 / 27.0_f64 * t639 * t3523;
    let t3526 = t591 * t3346;
    let t3527 = t590 * t3526;
    let t3529 = 4.0_f64 / 45.0_f64 * t587 * t3527;
    let t3530 = t1664 * t3342;
    let t3531 = t1661 * t3530;
    let t3533 = 4.0_f64 / 27.0_f64 * t587 * t3531;
    let t3534 = t2635 * t1017;
    (t3523, t3525, t3526, t3527, t3529, t3530, t3531, t3533, t3534)
}

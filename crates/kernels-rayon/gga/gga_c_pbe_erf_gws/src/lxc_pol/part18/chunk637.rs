//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 637/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk637(t3443: f64, t598: f64, t186: f64, t185: f64, t2790: f64, t997: f64, t198: f64, t3345: f64, t561: f64, t1017: f64, t1803: f64, t225: f64, t3379: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3444 = t598 * t3443;
    let t3445 = t186 * t3444;
    let t3447 = 2.0_f64 / 15.0_f64 * t185 * t3445;
    let t3449 = 8.0_f64 / 15.0_f64 * t2790 * t997;
    let t3450 = t198 * t3345;
    let t3451 = t186 * t3450;
    let t3453 = 4.0_f64 / 15.0_f64 * t561 * t3451;
    let t3454 = t1017 * t1017;
    let t3455 = t1803 * t3454;
    let t3456 = t186 * t3455;
    let t3458 = 4.0_f64 / 15.0_f64 * t185 * t3456;
    let t3459 = t3379 * t225;
    (t3444, t3445, t3447, t3449, t3450, t3451, t3453, t3454, t3455, t3456, t3458, t3459)
}

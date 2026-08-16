//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 635/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk635(t2579: f64, t950: f64, t1821: f64, t1820: f64, t1000: f64, t1017: f64, t1827: f64, t587: f64, t1006: f64, t1019: f64, t1663: f64, t3342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3410 = t2579 * t950;
    let t3411 = t1821 * t3410;
    let t3413 = 16.0_f64 / 45.0_f64 * t1820 * t3411;
    let t3414 = t1000 * t1017;
    let t3415 = t1827 * t3414;
    let t3417 = 8.0_f64 / 45.0_f64 * t587 * t3415;
    let t3419 = 4.0_f64 / 15.0_f64 * t1006 * t1019;
    let t3421 = t1663 * t3342;
    (t3410, t3411, t3413, t3414, t3415, t3417, t3419, t3421)
}

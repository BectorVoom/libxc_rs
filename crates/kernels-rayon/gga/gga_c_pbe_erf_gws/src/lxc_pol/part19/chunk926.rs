//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 926/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk926(t10378: f64, t562: f64, t1885: f64, t1820: f64, t3454: f64, t5175: f64, t610: f64, t587: f64, t2630: f64, t2784: f64, t1017: f64, t950: f64) -> (f64, f64, f64, f64) {
    let t10379 = t10378 * t562;
    let t10380 = t1885 * t10379;
    let t10382 = 8.0_f64 / 15.0_f64 * t1820 * t10380;
    let t10383 = t5175 * t3454;
    let t10384 = t10383 * t610;
    let t10385 = t1885 * t10384;
    let t10387 = 4.0_f64 / 5.0_f64 * t587 * t10385;
    let t10388 = t2630 * t2784;
    let t10389 = t1885 * t10388;
    let t10391 = 8.0_f64 / 15.0_f64 * t587 * t10389;
    let t10392 = t950 * t1017;
    (t10382, t10387, t10391, t10392)
}

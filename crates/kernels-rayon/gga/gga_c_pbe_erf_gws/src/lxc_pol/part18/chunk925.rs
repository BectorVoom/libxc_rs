//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 925/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk925(t3465: f64, t422: f64, t1809: f64, t639: f64, t2672: f64, t34: f64, t7194: f64, t3411: f64, t7136: f64, t5312: f64, t3345: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10353 = t3465 * t422;
    let t10354 = t1809 * t10353;
    let t10356 = 8.0_f64 / 15.0_f64 * t639 * t10354;
    let t10357 = t2672 * t34;
    let t10358 = t7194 * t10357;
    let t10360 = 32.0_f64 / 45.0_f64 * t639 * t10358;
    let t10362 = 16.0_f64 / 45.0_f64 * t7136 * t3411;
    let t10364 = 16.0_f64 / 45.0_f64 * t5312 * t3411;
    let t10365 = t597 * t3345;
    (t10353, t10356, t10357, t10360, t10362, t10364, t10365)
}

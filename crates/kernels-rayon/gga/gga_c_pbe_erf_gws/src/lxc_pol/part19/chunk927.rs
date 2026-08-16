//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 927/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk927(t10392: f64, t418: f64, t7063: f64, t7062: f64, t7069: f64, t5117: f64, t1044: f64, t954: f64, t422: f64, t7505: f64, t7115: f64, t626: f64, t7116: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10393 = t10392 * t418;
    let t10394 = t7063 * t10393;
    let t10396 = 16.0_f64 / 45.0_f64 * t7062 * t10394;
    let t10397 = t7069 * t10393;
    let t10399 = 8.0_f64 / 27.0_f64 * t7062 * t10397;
    let t10400 = 8.0_f64 / 135.0_f64 * t5117;
    let t10401 = t954 * t1044;
    let t10402 = t10401 * t422;
    let t10403 = t7505 * t10402;
    let t10405 = 16.0_f64 / 45.0_f64 * t7115 * t10403;
    let t10406 = t7116 * t626;
    (t10396, t10399, t10400, t10401, t10402, t10405, t10406)
}

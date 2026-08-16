//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 697/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk697(t1115: f64, t2244: f64, t2401: f64, t2408: f64, t2503: f64, t3047: f64, t3052: f64, t3055: f64, t3084: f64, t3086: f64, t3312: f64, t3321: f64, t335: f64, t3724: f64, t3733: f64, t3739: f64, t3744: f64, t3889: f64, t3893: f64, t3898: f64, t3903: f64, t3909: f64, t3913: f64, t3917: f64, t3921: f64, t833: f64, t844: f64) -> f64 {
    let t3928 = t335 * t3724 / 48.0_f64 - t1115 * t3052 / 24.0_f64 - t1115 * t3047 / 48.0_f64 - t3055 * t3733 / 96.0_f64 - 7.0_f64 / 144.0_f64 * t3084 + t2401 * t3739 / 16.0_f64 + t2408 * t3744 / 24.0_f64 + t2244 - t335 * t3889 / 96.0_f64 - t335 * t3893 / 48.0_f64 - t844 * t3898 / 48.0_f64 - 7.0_f64 / 144.0_f64 * t3321 - t844 * t3903 / 24.0_f64 + t335 * t3909 / 96.0_f64 + t3913 * t833 / 96.0_f64 + t3917 * t833 / 96.0_f64 + t3921 * t833 / 96.0_f64 + t1115 * t2503 / 48.0_f64 + 7.0_f64 / 72.0_f64 * t3312 + 7.0_f64 / 144.0_f64 * t3086;
    t3928
}

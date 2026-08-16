//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 861/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk861(t572: f64, t605: f64, t188: f64, t174: f64, t838: f64, t190: f64, t25: f64, t2718: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t5044: f64, t5241: f64, t5271: f64, t7327: f64, t7335: f64, t7337: f64, t7342: f64, t7347: f64, t7351: f64, t7356: f64, t7360: f64, t7364: f64) -> (f64, f64) {
    let t7365 = t605 * t572;
    let t7369 = t188 * t572;
    let t7371 = t174 * t838 * t7369;
    let t7373 = 0.13333333333333333333e-1_f64 * t25 * t7327 - 0.31992592592592592592e-1_f64 * t4941 + 0.7998148148148148148e-2_f64 * t4943 - 0.23994444444444444444e-1_f64 * t4945 + 0.11997222222222222222e-1_f64 * t4947 - t5241 + t7335 - 0.22222222222222222222e-2_f64 * t25 * t7337 - 0.29629629629629629629e-2_f64 * t25 * t7342 - 0.88888888888888888887e-2_f64 * t2718 * t7347 + 0.13333333333333333333e-1_f64 * t25 * t7351 + 0.53333333333333333332e-1_f64 * t2718 * t7356 - 0.39999999999999999999e-1_f64 * t25 * t7360 - t7364 - 0.13333333333333333333e-1_f64 * t190 * t5044 * t7365 - 0.71983333333333333334e-1_f64 * t7371 - t5271;
    (t7371, t7373)
}

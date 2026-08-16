//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 860/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk860(t1730: f64, t2737: f64, t4957: f64, t950: f64, t1403: f64, t1856: f64, t2775: f64, t401: f64, t1407: f64, t2560: f64, t4951: f64, t5264: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7324 = 8.0_f64 / 15.0_f64 * t1730 * t2737;
    let t7325 = t4957 * t950;
    let t7326 = t7325 * t1403;
    let t7327 = t1856 * t7326;
    let t7335 = 0.2962962962962962963e-2_f64 * t401 * t2775;
    let t7336 = t2560 * t1407;
    let t7337 = t1856 * t7336;
    let t7340 = t4951 * t950;
    let t7341 = t7340 * t1403;
    let t7342 = t5264 * t7341;
    (t7324, t7326, t7327, t7335, t7336, t7337, t7341, t7342)
}

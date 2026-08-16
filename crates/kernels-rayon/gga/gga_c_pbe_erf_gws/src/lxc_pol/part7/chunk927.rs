//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 927/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk927(t1829: f64, t5406: f64, t2735: f64, t561: f64, t563: f64, t1730: f64, t5116: f64, t17316: f64, t17318: f64, t17326: f64, t17328: f64, t17330: f64, t17335: f64, t17338: f64, t17341: f64) -> (f64, f64, f64, f64) {
    let t17343 = 16.0_f64 / 15.0_f64 * t5406 * t1829;
    let t17345 = t561 * t2735 * t563;
    let t17346 = 128.0_f64 / 405.0_f64 * t17345;
    let t17347 = t1730 * t5116;
    let t17348 = 32.0_f64 / 45.0_f64 * t17347;
    let t17349 = t17316 + t17318 + t17326 + t17328 + t17330 - t17335 + t17338 + t17341 - t17343 + t17346 - t17348;
    (t17343, t17346, t17348, t17349)
}

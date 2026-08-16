//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 849/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk849(t1076: f64, t1105: f64, t1123: f64, t2255: f64, t11668: f64, t3793: f64, t11581: f64, t11598: f64, t13349: f64, t13355: f64, t13357: f64, t13361: f64, t13363: f64, t13367: f64, t13373: f64, t13377: f64, t13379: f64, t13384: f64, t2277: f64, t2312: f64, t2343: f64) -> (f64, f64, f64, f64) {
    let t13385 = t1076 * t1105;
    let t13387 = t2255 * t1123 * t13385;
    let t13391 = t11668 * t3793 / 48.0_f64;
    let t13392 = t2343 * t13349 / 128.0_f64 + t13355 - t2312 * t13357 / 128.0_f64 - t13361 - t2277 * t13363 / 256.0_f64 - t13367 + t13373 + t13377 - t2343 * t13379 / 512.0_f64 - 7.0_f64 / 768.0_f64 * t11581 + 7.0_f64 / 96.0_f64 * t11598 - t13384 - t2312 * t13387 / 128.0_f64 - t13391;
    (t13385, t13387, t13391, t13392)
}

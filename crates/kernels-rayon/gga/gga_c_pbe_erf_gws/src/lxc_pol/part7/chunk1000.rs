//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1000/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1000(t17346: f64, t17348: f64, t17354: f64, t17359: f64, t17362: f64, t17364: f64, t17368: f64, t18256: f64, t18261: f64, t18267: f64, t18268: f64, t17372: f64, t17376: f64, t17378: f64, t17382: f64, t17384: f64, t17386: f64, t17391: f64, t17394: f64, t17397: f64, t17402: f64, t17404: f64, t17406: f64) -> (f64, f64) {
    let t18270 = 0.72933333333333333331e0_f64 * t18256 + t18261 + t18267 + 8.0_f64 / 3.0_f64 * t18268 + t17346 - t17348 + t17354 - t17359 - t17362 + t17364 - t17368;
    let t18271 = t17372 - t17376 + t17378 - t17382 - t17384 - t17386 + t17391 + t17394 + t17397 + t17402 - t17404 + t17406;
    (t18270, t18271)
}

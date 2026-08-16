//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1047/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1047(t2158: f64, t3219: f64, t3235: f64, t2323: f64, t3268: f64, t1113: f64, t904: f64, t6278: f64, t2277: f64, t3247: f64, t6275: f64, t6579: f64, t8960: f64, t8965: f64, t8969: f64, t8971: f64, t8973: f64, t8977: f64, t9478: f64, t9485: f64, t9490: f64) -> (f64, f64, f64, f64) {
    let t9494 = t3235 * t3219 * t2158;
    let t9498 = 7.0_f64 / 576.0_f64 * t2323 * t3268;
    let t9499 = t904 * t1113;
    let t9500 = t9499 * t6278;
    let t9503 = t8960 - t8965 - t2277 * t9478 / 768.0_f64 - t8969 + t2277 * t9485 / 384.0_f64 + 5.0_f64 / 384.0_f64 * t6579 * t9490 + 3.0_f64 / 512.0_f64 * t3247 * t9494 + t8971 + t9498 + t8973 - t8977 + t6275 * t9500 / 96.0_f64;
    (t9494, t9499, t9500, t9503)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1218/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1218(t13293: f64, t39181: f64, t44537: f64, t13398: f64, t13578: f64, t2255: f64, t2277: f64, t37800: f64, t3781: f64, t44246: f64, t45805: f64, t49315: f64, t49316: f64, t49318: f64, t49327: f64, t49329: f64, t49334: f64, t6685: f64, t9482: f64) -> (f64, f64, f64) {
    let t49344 = t39181 * t13293 / 16.0_f64;
    let t49345 = 7.0_f64 / 72.0_f64 * t44537;
    let t49346 = -t49315 - t49316 - t49318 + 3.0_f64 / 128.0_f64 * t6685 * t2255 * t3781 * t13398 - t49327 - t49329 - t49334 - t2277 * t9482 * t45805 * t37800 / 64.0_f64 + t2277 * t9482 * t13578 * t44246 / 64.0_f64 + t49344 - t49345;
    (t49344, t49345, t49346)
}

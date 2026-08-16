//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 854/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk854(t1123: f64, t13440: f64, t850: f64, t860: f64, t13086: f64, t858: f64, t886: f64, t884: f64, t11994: f64, t2255: f64, t3757: f64, t13394: f64, t13400: f64, t13407: f64, t13410: f64, t13416: f64, t13418: f64, t13423: f64, t13428: f64, t13433: f64, t13439: f64, t2277: f64, t2343: f64, t3247: f64, t6555: f64, t6685: f64, t902: f64, t9457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13442 = t850 * t1123 * t13440;
    let t13444 = t13442 * t860 / 96.0_f64;
    let t13446 = t886 * t858 * t13086;
    let t13448 = t884 * t13446 / 48.0_f64;
    let t13450 = t2255 * t11994 * t3757;
    let t13453 = t902 * t13394 / 768.0_f64 + 3.0_f64 / 256.0_f64 * t6685 * t13400 - t13407 - 5.0_f64 / 128.0_f64 * t2343 * t13410 - 119.0_f64 / 2304.0_f64 * t9457 + t13416 - 3.0_f64 / 128.0_f64 * t3247 * t13418 + t2277 * t13423 / 256.0_f64 - t6555 * t13428 / 128.0_f64 + 3.0_f64 / 512.0_f64 * t3247 * t13433 - t13439 + t13444 - t13448 - t2277 * t13450 / 768.0_f64;
    (t13442, t13444, t13446, t13448, t13450, t13453)
}

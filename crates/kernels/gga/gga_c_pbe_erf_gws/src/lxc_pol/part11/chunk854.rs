//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 854/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk854<F: Float>(t1123: F, t13440: F, t850: F, t860: F, t13086: F, t858: F, t886: F, t884: F, t11994: F, t2255: F, t3757: F, t13394: F, t13400: F, t13407: F, t13410: F, t13416: F, t13418: F, t13423: F, t13428: F, t13433: F, t13439: F, t2277: F, t2343: F, t3247: F, t6555: F, t6685: F, t902: F, t9457: F) -> (F, F, F, F, F, F) {
    let t13442 = t850 * t1123 * t13440;
    let t13444 = t13442 * t860 / F::new(96.0);
    let t13446 = t886 * t858 * t13086;
    let t13448 = t884 * t13446 / F::new(48.0);
    let t13450 = t2255 * t11994 * t3757;
    let t13453 = t902 * t13394 / F::new(768.0) + F::new(3.0) / F::new(256.0) * t6685 * t13400 - t13407 - F::new(5.0) / F::new(128.0) * t2343 * t13410 - F::new(119.0) / F::new(2304.0) * t9457 + t13416 - F::new(3.0) / F::new(128.0) * t3247 * t13418 + t2277 * t13423 / F::new(256.0) - t6555 * t13428 / F::new(128.0) + F::new(3.0) / F::new(512.0) * t3247 * t13433 - t13439 + t13444 - t13448 - t2277 * t13450 / F::new(768.0);
    (t13442, t13444, t13446, t13448, t13450, t13453)
}

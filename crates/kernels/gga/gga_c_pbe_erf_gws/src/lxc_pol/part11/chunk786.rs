//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 786/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk786<F: Float>(t11994: F, t2255: F, t3757: F, t13394: F, t13400: F, t13407: F, t13410: F, t13416: F, t13418: F, t13423: F, t13428: F, t13433: F, t13439: F, t13444: F, t13448: F, t2277: F, t2343: F, t3247: F, t6555: F, t6685: F, t902: F, t9457: F) -> (F, F) {
    let t13450 = t2255 * t11994 * t3757;
    let t13453 = t902 * t13394 / 768.0 + 3.0 / 256.0 * t6685 * t13400 - t13407 - 5.0 / 128.0 * t2343 * t13410 - 119.0 / 2304.0 * t9457 + t13416 - 3.0 / 128.0 * t3247 * t13418 + t2277 * t13423 / 256.0 - t6555 * t13428 / 128.0 + 3.0 / 512.0 * t3247 * t13433 - t13439 + t13444 - t13448 - t2277 * t13450 / 768.0;
    (t13450, t13453)
}

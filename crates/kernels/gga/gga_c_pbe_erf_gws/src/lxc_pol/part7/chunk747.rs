//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 747/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk747<F: Float>(t254: F, t6367: F, t906: F, t2266: F, t2277: F, t6517: F, t6522: F, t6528: F, t6532: F, t6537: F, t6540: F, t6544: F, t6545: F, t6548: F, t6555: F, t6557: F, t6565: F, t6572: F, t6575: F, t6579: F) -> (F, F, F) {
    let t6580 = t254 * t6367;
    let t6581 = t6580 * t906;
    let t6584 = 7.0 / 768.0 * t6517 - t6522 - t6528 + t6532 + t6537 - t6540 - t6544 + 7.0 / 768.0 * t6545 + 3.0 / 512.0 * t2266 * t6548 - t6555 * t6557 / 128.0 + t6565 + t6572 - t2277 * t6575 / 1536.0 + 5.0 / 128.0 * t6579 * t6581;
    (t6580, t6581, t6584)
}

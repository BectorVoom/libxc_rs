//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 795/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk795(t254: f64, t6367: f64, t906: f64, t2266: f64, t2277: f64, t6517: f64, t6522: f64, t6528: f64, t6532: f64, t6537: f64, t6540: f64, t6544: f64, t6545: f64, t6548: f64, t6555: f64, t6557: f64, t6565: f64, t6572: f64, t6575: f64, t6579: f64) -> (f64, f64, f64) {
    let t6580 = t254 * t6367;
    let t6581 = t6580 * t906;
    let t6584 = 7.0_f64 / 768.0_f64 * t6517 - t6522 - t6528 + t6532 + t6537 - t6540 - t6544 + 7.0_f64 / 768.0_f64 * t6545 + 3.0_f64 / 512.0_f64 * t2266 * t6548 - t6555 * t6557 / 128.0_f64 + t6565 + t6572 - t2277 * t6575 / 1536.0_f64 + 5.0_f64 / 128.0_f64 * t6579 * t6581;
    (t6580, t6581, t6584)
}

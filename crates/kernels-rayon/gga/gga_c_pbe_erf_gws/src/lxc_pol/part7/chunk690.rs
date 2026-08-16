//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 690/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk690(t5559: f64, t5560: f64, t1805: f64, t582: f64, t185: f64, t5504: f64, t5508: f64, t5512: f64, t5514: f64, t5518: f64, t5521: f64, t5526: f64, t5528: f64, t5532: f64, t5535: f64, t5538: f64, t5542: f64, t5547: f64, t5553: f64, t5555: f64, t5558: f64) -> (f64, f64, f64) {
    let t5562 = 0.15154381759259259259e-2_f64 * t5559 * t5560;
    let t5563 = t582 * t1805;
    let t5564 = t185 * t5563;
    let t5565 = 8.0_f64 / 15.0_f64 * t5564;
    let t5566 = -t5504 - t5508 + t5512 - t5514 + t5518 - t5521 - t5526 + t5528 - t5532 - t5535 + t5538 + t5542 - t5547 + t5553 - t5555 - t5558 + t5562 + t5565;
    (t5563, t5565, t5566)
}

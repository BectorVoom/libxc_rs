//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 690/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk690<F: Float>(t5559: F, t5560: F, t1805: F, t582: F, t185: F, t5504: F, t5508: F, t5512: F, t5514: F, t5518: F, t5521: F, t5526: F, t5528: F, t5532: F, t5535: F, t5538: F, t5542: F, t5547: F, t5553: F, t5555: F, t5558: F) -> (F, F, F) {
    let t5562 = F::cast_from(0.15154381759259259259e-2_f64) * t5559 * t5560;
    let t5563 = t582 * t1805;
    let t5564 = t185 * t5563;
    let t5565 = F::new(8.0) / F::new(15.0) * t5564;
    let t5566 = -t5504 - t5508 + t5512 - t5514 + t5518 - t5521 - t5526 + t5528 - t5532 - t5535 + t5538 + t5542 - t5547 + t5553 - t5555 - t5558 + t5562 + t5565;
    (t5563, t5565, t5566)
}

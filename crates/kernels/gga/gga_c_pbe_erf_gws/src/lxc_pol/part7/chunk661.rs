//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 661/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk661<F: Float>(t5548: F, t5550: F, t587: F, t1868: F, t579: F, t1672: F, t563: F, t561: F, t1: F, t1952: F, t119: F, t713: F, t1805: F, t582: F, t185: F, t5504: F, t5508: F, t5512: F, t5514: F, t5518: F, t5521: F, t5526: F, t5528: F, t5532: F, t5535: F, t5538: F, t5542: F, t5547: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5551 = t5548 * t5550;
    let t5553 = 8.0 / 15.0 * t587 * t5551;
    let t5555 = 2.0 / 5.0 * t579 * t1868;
    let t5556 = t1672 * t563;
    let t5557 = t561 * t5556;
    let t5558 = 8.0 / 45.0 * t5557;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    let t5562 = 0.15154381759259259259e-2 * t5559 * t5560;
    let t5563 = t582 * t1805;
    let t5564 = t185 * t5563;
    let t5565 = 8.0 / 15.0 * t5564;
    let t5566 = -t5504 - t5508 + t5512 - t5514 + t5518 - t5521 - t5526 + t5528 - t5532 - t5535 + t5538 + t5542 - t5547 + t5553 - t5555 - t5558 + t5562 + t5565;
    (t5551, t5553, t5555, t5556, t5558, t5559, t5560, t5563, t5565, t5566)
}

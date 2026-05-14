//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 731/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk731<F: Float>(t265: F, t5519: F, t1640: F, t649: F, t1661: F, t597: F, t1802: F, t590: F, t1672: F, t563: F, t561: F, t1: F, t1952: F, t119: F, t713: F, t1472: F, t168: F, t738: F) -> (F, F, F, F, F, F, F) {
    let t5521 = 8.0 / 405.0 * t265 * t5519;
    let t5522 = t1640 * t649;
    let t5543 = t1661 * t597;
    let t5548 = t590 * t1802;
    let t5556 = t1672 * t563;
    let t5557 = t561 * t5556;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    let t5562 = 0.15154381759259259259e-2 * t5559 * t5560;
    let t5574 = t168 * t1472 * t738;
    (t5521, t5522, t5543, t5548, t5557, t5562, t5574)
}

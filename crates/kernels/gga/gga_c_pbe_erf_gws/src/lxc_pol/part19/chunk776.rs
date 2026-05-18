//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 776/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk776<F: Float>(t1672: F, t563: F, t561: F, t1: F, t1952: F, t119: F, t713: F, t1472: F, t168: F, t738: F, t1457: F, t242: F) -> (F, F, F, F) {
    let t5556 = t1672 * t563;
    let t5557 = t561 * t5556;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    let t5562 = F::new(0.15154381759259259259e-2) * t5559 * t5560;
    let t5574 = t168 * t1472 * t738;
    let t5582 = t1457 * t242;
    (t5557, t5562, t5574, t5582)
}

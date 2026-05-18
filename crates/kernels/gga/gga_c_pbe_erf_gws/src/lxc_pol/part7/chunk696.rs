//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 696/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk696<F: Float>(t168: F, t5589: F, t286: F, t159: F, t285: F, t4562: F, t4353: F, t4356: F, t4361: F, t4368: F, t4371: F, t4374: F) -> (F, F, F, F) {
    let t5631 = t168 * t5589;
    let t5633 = F::new(0.19513566535229733338e0) * t5631 * t286;
    let t5636 = t4562 * t159 * t285;
    let t5645 = F::new(4.0) / F::new(27.0) * t4353 - t4356 / F::new(3.0) + t4361 / F::new(3.0) + F::new(4.0) / F::new(27.0) * t4368 - t4371 / F::new(3.0) + t4374 / F::new(3.0);
    (t5631, t5633, t5636, t5645)
}

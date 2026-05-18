//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1338/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1338<F: Float>(t51285: F, t51293: F, t51302: F, t51315: F, t51330: F, t51332: F, t54215: F, t54217: F, t54219: F, t54224: F, t54226: F, t54231: F) -> F {
    let t55546 = -t54215 / F::new(48.0) + t54217 / F::new(192.0) + t54219 / F::new(384.0) - F::new(7.0) / F::new(576.0) * t51285 + F::new(7.0) / F::new(36.0) * t51293 - F::new(7.0) / F::new(192.0) * t51302 - t54224 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t54226 - F::new(7.0) / F::new(288.0) * t51315 - t54231 / F::new(24.0) + F::new(7.0) / F::new(144.0) * t51330 - F::new(7.0) / F::new(576.0) * t51332;
    t55546
}

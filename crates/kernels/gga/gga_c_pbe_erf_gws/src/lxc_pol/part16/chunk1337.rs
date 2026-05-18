//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1337/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1337<F: Float>(t54198: F, t54183: F, t54186: F, t54188: F, t54190: F, t54192: F, t54194: F, t54196: F, t54201: F, t54203: F, t54205: F, t54207: F, t54209: F) -> F {
    let t55524 = F::new(7.0) / F::new(288.0) * t54198;
    let t55530 = t54183 / F::new(48.0) + t54186 / F::new(24.0) + t54188 / F::new(12.0) + t54190 / F::new(48.0) + t54192 / F::new(64.0) + t54194 / F::new(64.0) - t54196 / F::new(16.0) - t55524 + t54201 / F::new(48.0) - t54203 / F::new(24.0) - t54205 / F::new(48.0) - t54207 / F::new(24.0) + t54209 / F::new(24.0);
    t55530
}

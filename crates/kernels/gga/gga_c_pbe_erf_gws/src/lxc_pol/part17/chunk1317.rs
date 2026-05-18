//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1317/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1317<F: Float>(t51341: F, t51358: F, t54237: F, t54239: F, t54241: F, t54246: F, t54248: F, t54251: F, t54255: F, t54258: F, t54260: F, t54261: F) -> F {
    let t54263 = t54237 - t54239 - F::new(7.0) / F::new(72.0) * t51341 + t54241 / F::new(48.0) + t54246 / F::new(24.0) + t54248 / F::new(192.0) - F::new(7.0) / F::new(288.0) * t51358 - t54251 / F::new(16.0) - t54255 / F::new(48.0) + t54258 - t54260 - t54261 / F::new(768.0);
    t54263
}

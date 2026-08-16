//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1317/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1317<F: Float>(t51341: F, t51358: F, t54237: F, t54239: F, t54241: F, t54246: F, t54248: F, t54251: F, t54255: F, t54258: F, t54260: F, t54261: F) -> F {
    let t54263 = t54237 - t54239 - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51341 + t54241 / F::cast_from(48.0_f64) + t54246 / F::cast_from(24.0_f64) + t54248 / F::cast_from(192.0_f64) - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51358 - t54251 / F::cast_from(16.0_f64) - t54255 / F::cast_from(48.0_f64) + t54258 - t54260 - t54261 / F::cast_from(768.0_f64);
    t54263
}

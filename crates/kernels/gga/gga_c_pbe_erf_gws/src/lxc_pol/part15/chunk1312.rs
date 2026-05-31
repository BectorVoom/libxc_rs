//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1312/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1312<F: Float>(t51306: F, t9500: F, t54183: F, t54186: F, t54188: F, t54190: F, t54192: F, t54194: F, t54196: F, t54199: F, t54201: F, t54203: F, t54205: F, t54207: F) -> F {
    let t54209 = t51306 * t9500;
    let t54211 = t54183 / F::cast_from(96.0_f64) + t54186 / F::cast_from(48.0_f64) + t54188 / F::cast_from(24.0_f64) + t54190 / F::cast_from(96.0_f64) + t54192 / F::cast_from(128.0_f64) + t54194 / F::cast_from(128.0_f64) - t54196 / F::cast_from(32.0_f64) - t54199 + t54201 / F::cast_from(96.0_f64) - t54203 / F::cast_from(48.0_f64) - t54205 / F::cast_from(96.0_f64) - t54207 / F::cast_from(48.0_f64) + t54209 / F::cast_from(48.0_f64);
    t54211
}

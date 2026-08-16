//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1314/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1314<F: Float>(t51285: F, t51293: F, t51302: F, t51315: F, t51330: F, t51332: F, t54215: F, t54217: F, t54219: F, t54224: F, t54226: F, t54231: F) -> F {
    let t54235 = -t54215 / F::cast_from(96.0_f64) + t54217 / F::cast_from(384.0_f64) + t54219 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51285 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t51293 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t51302 - t54224 / F::cast_from(192.0_f64) + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t54226 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t51315 - t54231 / F::cast_from(48.0_f64) + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t51330 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t51332;
    t54235
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1388/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1388<F: Float>(t54293: F, t54294: F, t54305: F, t55580: F, t57127: F, t57130: F, t57132: F, t57134: F, t57138: F, t57140: F, t57142: F, t57144: F, t57146: F) -> F {
    let t58730 = -t57127 / F::cast_from(2.0_f64) + t57130 / F::cast_from(4.0_f64) + t57132 / F::cast_from(24.0_f64) - t57134 / F::cast_from(192.0_f64) - t54293 - t54294 + t57138 / F::cast_from(12.0_f64) + t55580 - F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t54305 - t57140 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t57142 - F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t57144 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t57146;
    t58730
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1388/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1388<F: Float>(t54293: F, t54294: F, t54305: F, t55580: F, t57127: F, t57130: F, t57132: F, t57134: F, t57138: F, t57140: F, t57142: F, t57144: F, t57146: F) -> F {
    let t58730 = -t57127 / F::new(2.0) + t57130 / F::new(4.0) + t57132 / F::new(24.0) - t57134 / F::new(192.0) - t54293 - t54294 + t57138 / F::new(12.0) + t55580 - F::new(119.0) / F::new(864.0) * t54305 - t57140 / F::new(384.0) - F::new(7.0) / F::new(72.0) * t57142 - F::new(7.0) / F::new(24.0) * t57144 + F::new(7.0) / F::new(72.0) * t57146;
    t58730
}

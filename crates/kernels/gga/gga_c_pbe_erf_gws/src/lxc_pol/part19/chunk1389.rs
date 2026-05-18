//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1389/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1389<F: Float>(t54321: F, t55591: F, t55593: F, t57151: F, t57154: F, t57156: F, t57158: F, t57160: F, t57162: F, t57164: F, t57166: F, t57168: F) -> F {
    let t58742 = t57151 / F::new(96.0) - t55591 - t54321 + t57154 / F::new(24.0) - t55593 - t57156 / F::new(24.0) - t57158 / F::new(48.0) + F::new(7.0) / F::new(72.0) * t57160 - t57162 / F::new(48.0) - t57164 / F::new(48.0) - t57166 / F::new(48.0) - t57168 / F::new(384.0);
    t58742
}

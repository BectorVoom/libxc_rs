//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1361/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1361<F: Float>(t51870: F, t51877: F, t53784: F, t53971: F, t53976: F, t53977: F, t53980: F, t53986: F, t54430: F, t55751: F, t57386: F, t57390: F, t57393: F, t57395: F, t57398: F, t8793: F) -> F {
    let t57401 = t57386 / F::new(192.0) - t8793 * t53784 / F::new(8.0) - t53971 + t53976 - t57390 / F::new(16.0) - F::new(35.0) / F::new(216.0) * t53977 + t57393 / F::new(24.0) + t53980 + t53986 + t57395 / F::new(48.0) - t55751 + t54430 - t57398 / F::new(48.0) - t51870 + F::new(35.0) / F::new(432.0) * t51877;
    t57401
}

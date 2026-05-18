//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1294/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1294<F: Float>(t53994: F, t53996: F, t53998: F, t54000: F, t54002: F, t54004: F, t54006: F, t54008: F, t54010: F, t54012: F, t54015: F, t54016: F) -> F {
    let t54018 = t53994 / F::new(32.0) + t53996 / F::new(24.0) + t53998 / F::new(24.0) - t54000 / F::new(192.0) - t54002 / F::new(384.0) + t54004 / F::new(24.0) - t54006 / F::new(48.0) - t54008 / F::new(96.0) + t54010 / F::new(16.0) - t54012 / F::new(48.0) + t54015 + t54016 / F::new(192.0);
    t54018
}

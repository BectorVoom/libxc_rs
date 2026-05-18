//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1384/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1384<F: Float>(t55524: F, t57028: F, t57031: F, t57036: F, t57038: F, t57040: F, t57042: F, t57044: F, t57046: F, t57048: F, t57050: F, t57052: F, t57054: F) -> F {
    let t58683 = t57028 / F::new(24.0) - t57031 / F::new(24.0) + t57036 / F::new(24.0) - t57038 / F::new(24.0) - t57040 / F::new(24.0) - t57042 / F::new(192.0) - t55524 + t57044 / F::new(4.0) - t57046 / F::new(24.0) - t57048 / F::new(48.0) + t57050 / F::new(96.0) + t57052 / F::new(64.0) - t57054 / F::new(12.0);
    t58683
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 230/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk230<F: Float>(t670: F, t672: F, t395: F, t401: F, t7: F) -> (F, F, F, F) {
    let t674 = 0.10821041362364843377e0 * t670 * t672;
    let t677 = 0.4125e0 * t395 - t401 / 6.0;
    let t678 = t677 * M_PI;
    let t679 = t678 * t7;
    (t674, t677, t678, t679)
}

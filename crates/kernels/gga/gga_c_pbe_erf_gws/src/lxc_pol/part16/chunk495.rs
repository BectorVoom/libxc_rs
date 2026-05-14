//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 495/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk495<F: Float>(t2158: F, t858: F, t867: F, t2155: F, t837: F, t863: F, t864: F) -> (F, F, F, F) {
    let t2159 = t858 * t2158;
    let t2160 = t867 * t2159;
    let t2162 = t2155 * t2160 / 48.0;
    let t2164 = t863 * t864 * t837;
    (t2159, t2160, t2162, t2164)
}

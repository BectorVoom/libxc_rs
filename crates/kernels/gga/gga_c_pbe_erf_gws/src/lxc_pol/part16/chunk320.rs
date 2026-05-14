//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 320/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk320<F: Float>(t353: F, t939: F, t338: F, t335: F, t827: F, t833: F, t842: F, t844: F, t847: F, t894: F) -> (F, F, F) {
    let t940 = t353 * t939;
    let t941 = t338 * t940;
    let t944 = t827 * t833 / 96.0 - t842 - t844 * t847 / 48.0 + t335 * t894 / 96.0 - t335 * t941 / 96.0;
    (t940, t941, t944)
}

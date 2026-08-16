//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 314/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk314<F: Float>(t369: F, t923: F, t371: F, t364: F, t366: F, t899: F, t900: F) -> (F, F, F, F) {
    let t924 = t923 * t369;
    let t925 = t924 * t371;
    let t927 = F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t364 * t925;
    let t929 = t899 * t900 * t366;
    (t924, t925, t927, t929)
}

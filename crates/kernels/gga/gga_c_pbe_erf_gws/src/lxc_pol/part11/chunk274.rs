//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 274/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk274<F: Float>(t191: F, t922: F, t369: F, t371: F, t364: F, t366: F, t899: F, t900: F) -> (F, F, F) {
    let t923 = t922 * t191;
    let t924 = t923 * t369;
    let t925 = t924 * t371;
    let t927 = F::new(7.0) / F::new(4608.0) * t364 * t925;
    let t929 = t899 * t900 * t366;
    (t923, t927, t929)
}

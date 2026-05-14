//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1164/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1164<F: Float>(t22393: F, t2409: F, t3959: F, t898: F, t911: F, t3973: F, t3972: F, t53800: F, t8884: F, t13953: F, t14787: F, t13796: F, t13859: F, t2171: F, t52921: F, t14423: F, t6220: F) -> (F, F, F, F, F) {
    let t54496 = t3959 * t2409 * t22393;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54502 = t3972 * t54499 * t8884 * t53800;
    let t54504 = t13953 * t14787;
    let t54505 = 7.0 / 144.0 * t54504;
    let t54508 = t13859 * t13796 * t52921 * t2171;
    let t54512 = t13859 * t13796 * t14423 * t6220;
    (t54496, t54502, t54505, t54508, t54512)
}

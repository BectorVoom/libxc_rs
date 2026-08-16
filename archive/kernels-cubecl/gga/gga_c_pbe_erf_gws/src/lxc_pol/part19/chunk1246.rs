//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1246/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1246<F: Float>(t13972: F, t14608: F, t898: F, t911: F, t3973: F, t13953: F, t14787: F, t14781: F, t14001: F, t3062: F, t14772: F, t14466: F) -> (F, F, F, F, F, F, F) {
    let t54491 = t13972 * t14608;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54504 = t13953 * t14787;
    let t54531 = t13953 * t14781;
    let t54535 = t14001 * t3062;
    let t54537 = t14001 * t14772;
    let t54566 = t14001 * t14466;
    (t54491, t54499, t54504, t54531, t54535, t54537, t54566)
}

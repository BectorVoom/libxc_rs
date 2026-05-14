//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1128/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1128<F: Float>(t14767: F, t2397: F, t1134: F, t13776: F, t2410: F, t50956: F, t3959: F, t8756: F, t13972: F, t14608: F, t22393: F, t2409: F, t898: F, t911: F, t3973: F, t3972: F, t53800: F, t8884: F) -> (F, F, F, F, F, F) {
    let t54465 = t14767 * t2397;
    let t54473 = t13776 * t50956 * t1134 * t2410;
    let t54484 = t3959 * t8756;
    let t54491 = t13972 * t14608;
    let t54496 = t3959 * t2409 * t22393;
    let t54498 = t911 * t898;
    let t54499 = t3973 * t54498;
    let t54502 = t3972 * t54499 * t8884 * t53800;
    (t54465, t54473, t54484, t54491, t54496, t54502)
}

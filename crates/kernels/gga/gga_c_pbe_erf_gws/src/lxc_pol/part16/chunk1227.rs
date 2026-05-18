//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1227/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1227<F: Float>(t1162: F, t13917: F, t3223: F, t361: F, t874: F, t2081: F, t28672: F, t3972: F, t3975: F, t6472: F, t13808: F, t14698: F) -> (F, F, F) {
    let t53053 = t13917 * t361 * t1162 * t874 * t3223;
    let t53058 = t3972 * t3975 * t28672 * t6472 * t2081;
    let t53060 = t13808 * t14698;
    (t53053, t53058, t53060)
}

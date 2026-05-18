//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 871/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk871<F: Float>(t3028: F, t817: F, t1109: F, t2106: F, t1140: F, t6480: F, t1125: F, t6616: F, t2145: F, t3039: F, t19: F, t931: F) -> (F, F, F, F, F, F) {
    let t9147 = t3028 * t817;
    let t9150 = t1109 * t2106;
    let t9176 = t6480 * t1140;
    let t9182 = t1125 * t6616;
    let t9188 = t3039 * t2145;
    let t9239 = t931 * t19;
    (t9147, t9150, t9176, t9182, t9188, t9239)
}

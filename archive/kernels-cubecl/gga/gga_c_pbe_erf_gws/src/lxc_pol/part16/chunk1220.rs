//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1220/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1220<F: Float>(t13917: F, t4149: F, t9521: F, t14765: F, t2118: F, t3074: F, t6778: F, t13808: F, t14754: F, t3972: F, t3975: F, t9416: F) -> (F, F, F, F) {
    let t52889 = t13917 * t4149 * t9521;
    let t52893 = t3074 * t2118 * t14765 * t6778;
    let t52901 = t13808 * t14754;
    let t52904 = t3972 * t3975 * t9416;
    (t52889, t52893, t52901, t52904)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 988/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk988<F: Float>(t1176: F, t2333: F, t1180: F, t13888: F, t938: F, t353: F, t859: F) -> (F, F, F, F) {
    let t13893 = t1176 * t2333;
    let t13894 = t13893 * t1180;
    let t13895 = 119.0 / 13824.0 * t13894;
    let t13909 = t13888 * t938;
    let t13910 = t353 * t13909;
    let t13911 = t859 * t13910;
    (t13893, t13895, t13909, t13911)
}

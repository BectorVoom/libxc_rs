//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1087/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1087<F: Float>(t1178: F, t2418: F, t371: F, t1177: F, t2338: F, t3975: F, t3972: F, t13888: F, t938: F, t353: F, t859: F) -> (F, F, F, F, F, F, F) {
    let t13903 = t371 * t1178 * t2418;
    let t13904 = t1177 * t13903;
    let t13906 = t3975 * t2338;
    let t13907 = t3972 * t13906;
    let t13909 = t13888 * t938;
    let t13910 = t353 * t13909;
    let t13911 = t859 * t13910;
    (t13903, t13904, t13906, t13907, t13909, t13910, t13911)
}

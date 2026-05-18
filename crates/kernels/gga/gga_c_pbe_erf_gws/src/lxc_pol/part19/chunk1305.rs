//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1305/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1305<F: Float>(t11625: F, t14007: F, t11521: F, t14498: F, t11930: F, t14015: F, t11750: F, t51351: F, t11444: F, t11938: F, t11427: F, t51306: F) -> (F, F, F, F, F, F, F) {
    let t56894 = t14007 * t11625;
    let t56896 = t14498 * t11521;
    let t56898 = t14015 * t11930;
    let t56901 = t51351 * t11750;
    let t56903 = t51351 * t11444;
    let t56905 = t14498 * t11938;
    let t56910 = t51306 * t11427;
    (t56894, t56896, t56898, t56901, t56903, t56905, t56910)
}

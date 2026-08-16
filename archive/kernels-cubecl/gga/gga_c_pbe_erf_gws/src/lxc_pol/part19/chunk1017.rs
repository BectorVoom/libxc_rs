//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1017/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1017<F: Float>(t1144: F, t3307: F, t338: F, t328: F, t3780: F, t2306: F, t3074: F, t1162: F, t3200: F, t3717: F, t938: F, t2376: F, t2409: F) -> (F, F, F, F, F, F) {
    let t11384 = t338 * t1144 * t3307;
    let t11387 = t3780 * t328;
    let t11388 = t2306 * t11387;
    let t11389 = t3074 * t11388;
    let t11393 = t338 * t3200 * t1162;
    let t11396 = t3717 * t938;
    let t11398 = t2409 * t2376 * t11396;
    (t11384, t11387, t11389, t11393, t11396, t11398)
}

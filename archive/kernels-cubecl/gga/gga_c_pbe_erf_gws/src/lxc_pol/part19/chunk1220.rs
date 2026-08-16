//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1220/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1220<F: Float>(t3958: F, t6659: F, t26730: F, t353: F, t859: F, t332: F, t6158: F, t4408: F, t1176: F, t2298: F, t923: F, t51649: F, t867: F) -> (F, F, F, F, F, F) {
    let t51898 = t3958 * t6659;
    let t51913 = t859 * t353 * t26730;
    let t51916 = t6158 * t332;
    let t51922 = t4408 * t332;
    let t51963 = t1176 * t923 * t2298;
    let t51966 = t51649 * t867;
    (t51898, t51913, t51916, t51922, t51963, t51966)
}

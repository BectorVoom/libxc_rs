//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1220/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1220(t3958: f64, t6659: f64, t26730: f64, t353: f64, t859: f64, t332: f64, t6158: f64, t4408: f64, t1176: f64, t2298: f64, t923: f64, t51649: f64, t867: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51898 = t3958 * t6659;
    let t51913 = t859 * t353 * t26730;
    let t51916 = t6158 * t332;
    let t51922 = t4408 * t332;
    let t51963 = t1176 * t923 * t2298;
    let t51966 = t51649 * t867;
    (t51898, t51913, t51916, t51922, t51963, t51966)
}

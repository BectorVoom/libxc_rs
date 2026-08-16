//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1211/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1211(t13903: f64, t3979: f64, t3958: f64, t6659: f64, t14109: f64, t840: f64, t332: f64, t6158: f64, t19911: f64, t353: f64, t859: f64, t4408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t51896 = t3979 * t13903;
    let t51898 = t3958 * t6659;
    let t51906 = t840 * t14109;
    let t51916 = t6158 * t332;
    let t51919 = t859 * t353 * t19911;
    let t51922 = t4408 * t332;
    (t51896, t51898, t51906, t51916, t51919, t51922)
}

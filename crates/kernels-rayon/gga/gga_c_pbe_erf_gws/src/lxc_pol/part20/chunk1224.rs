//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1224/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1224(t51869: f64, t13987: f64, t894: f64, t3958: f64, t6659: f64, t26730: f64, t353: f64, t859: f64, t332: f64, t6158: f64, t4408: f64, t1195: f64, t6729: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51870 = 595.0_f64 / 10368.0_f64 * t51869;
    let t51877 = t13987 * t894;
    let t51898 = t3958 * t6659;
    let t51913 = t859 * t353 * t26730;
    let t51916 = t6158 * t332;
    let t51922 = t4408 * t332;
    let t51957 = 455.0_f64 / 1296.0_f64 * t6729 * t1195;
    (t51870, t51877, t51898, t51913, t51916, t51922, t51957)
}

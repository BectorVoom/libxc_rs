//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1239/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1239(t1176: f64, t14639: f64, t6365: f64, t923: f64, t1112: f64, t13918: f64, t361: f64) -> (f64, f64, f64) {
    let t53424 = t1176 * t923 * t6365 * t14639;
    let t53425 = 35.0_f64 / 576.0_f64 * t53424;
    let t53446 = t13918 * t1112;
    let t53447 = t361 * t53446;
    (t53425, t53446, t53447)
}

//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1287/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1287(t1134: f64, t3068: f64, t3972: f64, t53240: f64, t3902: f64, t4386: f64, t13792: f64, t11745: f64, t13917: f64, t53447: f64, t11534: f64, t13919: f64) -> (f64, f64, f64, f64) {
    let t56511 = t3972 * t53240 * t1134 * t3068;
    let t56513 = t4386 * t3902;
    let t56514 = t13792 * t56513;
    let t56520 = t13917 * t53447 * t11745;
    let t56525 = t13917 * t13919 * t11534;
    (t56511, t56514, t56520, t56525)
}

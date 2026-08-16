//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 580/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk580(t3082: f64, t370: f64, t35: f64, t365: f64, t612: f64, t364: f64, t354: f64, t1032: f64, t1036: f64, t1004: f64, t1031: f64, t1044: f64, t248: f64, t2776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3084 = t370 * t3082 / 13824.0_f64;
    let t3087 = 1.0_f64 / t35 / t365 / t612;
    let t3088 = t364 * t3087;
    let t3089 = t354 * t3088;
    let t3092 = t1032 * t1036;
    let t3094 = t1004 * t1031;
    let t3098 = t248 * t1044 * t2776;
    (t3084, t3087, t3088, t3089, t3092, t3094, t3098)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1980/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1980(t3800: f64, t12625: f64, t458: f64, t13180: f64, t493: f64, t10308: f64, t599: f64, t90: f64, t29: f64, t560: f64, t9655: f64, t1389: f64, t268: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44125 = t3800 * t3800;
    let t44126 = 1.0_f64 / t44125;
    let t44841 = 1.0_f64 / t12625 / t458;
    let t45551 = 1.0_f64 / t13180 / t493;
    let t45963 = t599 * t10308;
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = 1.0_f64 / t9655 / t560;
    let t46808 = t1389 * t268;
    (t44126, t44841, t45551, t45963, t45972, t46361, t46808)
}

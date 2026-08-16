//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 783/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk783(t290: f64, t7884: f64, t7244: f64, t7484: f64, t35383: f64, t7473: f64, t7450: f64, t34884: f64, t7751: f64, t507: f64, t7191: f64, t275: f64, t7889: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36424 = t290 * t7884;
    let t36448 = t7244 * t7484;
    let t36450 = t35383 * t7473;
    let t36453 = t7244 * t7450;
    let t36464 = t34884 * t7751;
    let t36471 = t507 * t7191;
    let t36475 = t275 * t7889;
    (t36424, t36448, t36450, t36453, t36464, t36471, t36475)
}

//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 785/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk785(t36271: f64, t7204: f64, t36277: f64, t7192: f64, t7244: f64, t7484: f64, t35383: f64, t7473: f64, t7450: f64, t34884: f64, t7751: f64, t507: f64, t7191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36416 = t7204 * t36271;
    let t36418 = t7192 * t36277;
    let t36448 = t7244 * t7484;
    let t36450 = t35383 * t7473;
    let t36453 = t7244 * t7450;
    let t36464 = t34884 * t7751;
    let t36471 = t507 * t7191;
    (t36416, t36418, t36448, t36450, t36453, t36464, t36471)
}

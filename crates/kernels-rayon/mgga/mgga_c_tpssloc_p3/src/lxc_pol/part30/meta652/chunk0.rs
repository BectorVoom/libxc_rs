//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2066/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2066(t7611: f64, t82716: f64, t25550: f64, t82822: f64, t23384: f64, t25476: f64, t25467: f64, t25459: f64, t7604: f64, t82632: f64, t25723: f64, t88810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t89310 = t82716 * t7611;
    let t89327 = 0.18277045187202515961e-2_f64 * t82822 * t25550;
    let t89329 = 0.18277045187202515961e-2_f64 * t23384 * t25476;
    let t89360 = 0.54831135561607547884e-2_f64 * t23384 * t25467;
    let t89362 = 0.54831135561607547884e-2_f64 * t23384 * t25459;
    let t89366 = t82632 * t7604;
    let t89369 = 0.24369393582936687948e-2_f64 * t88810 * t25723;
    (t89310, t89327, t89329, t89360, t89362, t89366, t89369)
}

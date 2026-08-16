//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 912/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk912(t3138: f64, t9555: f64, t458: f64, t8556: f64, t1108: f64, t8550: f64, t1106: f64, t453: f64, t3054: f64, t450: f64, t3049: f64, t2845: f64, t390: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9573 = t3138 * t9555;
    let t9605 = t458 * t8556;
    let t9607 = t8550 * t1108 * t9605;
    let t9614 = t1106 * t1106;
    let t9615 = 1.0_f64 / t9614;
    let t9616 = t9615 * t453;
    let t9618 = t8550 * t9616 * t9605;
    let t9619 = t3054 * t450;
    let t9626 = t8550 * t3049 * t9605;
    let t9637 = 1.0_f64 / t390 / t2845;
    (t9573, t9607, t9615, t9618, t9619, t9626, t9637)
}

//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 167/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk167(t513: f64, t83: f64, t106: f64, t101: f64, t477: f64, t479: f64, t483: f64, t488: f64) -> (f64, f64, f64, f64, f64) {
    let t514 = t83 * t513;
    let t518 = t106 * t106;
    let t519 = 1.0_f64 / t518;
    let t520 = t101 * t519;
    let t525 = -0.1176575e1_f64 * t477 - 0.516475e0_f64 * t479 - 0.2103875e0_f64 * t483 - 0.104195e0_f64 * t488;
    (t514, t518, t519, t520, t525)
}

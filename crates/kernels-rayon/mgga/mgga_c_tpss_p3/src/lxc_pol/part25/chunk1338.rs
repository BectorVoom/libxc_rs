//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1338/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1338(t1792: f64, t19349: f64, t19404: f64, t19408: f64, t20246: f64, t6077: f64, t62307: f64, t62309: f64, t62345: f64, t67352: f64, t67389: f64, t67391: f64, t67441: f64, t69108: f64, t69111: f64, t69114: f64, t69143: f64) -> f64 {
    let t71431 = -t67389 - t67391 - 440.0_f64 / 27.0_f64 * t62307 - 176.0_f64 / 27.0_f64 * t62309 + 20.0_f64 / 3.0_f64 * t19349 * t67352 - 70.0_f64 * t62345 * t69143 - 10.0_f64 / 3.0_f64 * t67441 * t6077 - 10.0_f64 / 3.0_f64 * t20246 * t19404 - 10.0_f64 / 3.0_f64 * t20246 * t19408 - 4.0_f64 / 3.0_f64 * t69108 * t1792 - 4.0_f64 / 3.0_f64 * t69111 * t1792 - 4.0_f64 / 3.0_f64 * t69114 * t1792;
    t71431
}

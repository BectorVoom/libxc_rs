//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1318/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1318(t1509: f64, t232: f64, t25119: f64, t5527: f64, t815: f64, t20947: f64, t841: f64, t20870: f64, t6605: f64, t20896: f64, t6621: f64, t20963: f64, t23048: f64) -> (f64, f64, f64, f64, f64) {
    let t105387 = t25119 * t815 * t5527 * t1509 * t232;
    let t105390 = t25119 * t841 * t20947;
    let t105393 = t6605 * t815 * t20870;
    let t105396 = t6621 * t20896;
    let t105402 = t23048 * t20963;
    (t105387, t105390, t105393, t105396, t105402)
}

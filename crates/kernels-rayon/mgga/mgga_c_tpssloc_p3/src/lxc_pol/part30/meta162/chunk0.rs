//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 844/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk844(t4347: f64, t882: f64, t123: f64, t2765: f64, t2766: f64, t4335: f64, t4340: f64, t4345: f64, t291: f64, t1543: f64, t892: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4348 = t882 * t4347;
    let t4349 = t123 * t4348;
    let t4351 = t2765 + 0.5936111111111111111e-2_f64 * t2766 + 0.5936111111111111111e-2_f64 * t4335 - 0.11872222222222222222e-1_f64 * t4340 + 0.35616666666666666666e-1_f64 * t4345 - 0.17808333333333333333e-1_f64 * t4349;
    let t4353 = 0.621814e-1_f64 * t4351 * t291;
    let t4354 = t1543 * t892;
    let t4356 = 1.0_f64 * t4354 * t914;
    (t4348, t4349, t4351, t4353, t4354, t4356)
}

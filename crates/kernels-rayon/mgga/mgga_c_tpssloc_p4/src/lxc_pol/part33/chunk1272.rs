//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1272/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1272(t1358: f64, t28088: f64, t22852: f64, t3792: f64, t80798: f64, t97312: f64, t22705: f64, t236: f64, t550: f64, t6414: f64, t22765: f64, t6417: f64) -> (f64, f64, f64, f64) {
    let t97363 = t28088 * t1358;
    let t97367 = t22852 * t80798 * t97312 * t3792;
    let t97372 = t22852 * t22705 * t236 * t6414 * t550;
    let t97378 = t22765 * t6417;
    (t97363, t97367, t97372, t97378)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1190/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1190(t20475: f64, t26309: f64, t20460: f64, t22833: f64, t20454: f64, t26233: f64, t6422: f64, t20565: f64, t6952: f64, t20556: f64, t6945: f64, t1827: f64, t97246: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t107065 = t26309 * t20475;
    let t107067 = t22833 * t20460;
    let t107070 = t22833 * t20454;
    let t107074 = t26233 * t6422;
    let t107077 = t6952 * t20565;
    let t107084 = t6945 * t20556;
    let t107086 = t97246 * t1827;
    (t107065, t107067, t107070, t107074, t107077, t107084, t107086)
}

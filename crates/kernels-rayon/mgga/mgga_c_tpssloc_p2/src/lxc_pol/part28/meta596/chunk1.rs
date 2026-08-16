//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1894/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1894(t12571: f64, t608: f64, t33: f64, t46099: f64, t2244: f64, t3953: f64, t1410: f64, t9239: f64, t2241: f64, t72: f64, t7431: f64, t12648: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90114 = t12571 * t608;
    let t90121 = t46099 * t33;
    let t90132 = t3953 * t2244;
    let t90137 = t9239 * t1410;
    let t90141 = t72 * t7431 * t2241;
    let t90150 = t605 * t12648;
    (t90114, t90121, t90132, t90137, t90141, t90150)
}

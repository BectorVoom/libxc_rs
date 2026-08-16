//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1389/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1389(t6396: f64, t91100: f64, t20450: f64, t22833: f64, t6390: f64, t91388: f64, t1339: f64, t1824: f64, t22827: f64, t550: f64, t6347: f64, t1799: f64, t6414: f64) -> (f64, f64, f64, f64, f64) {
    let t107145 = t91100 * t6396;
    let t107147 = t22833 * t20450;
    let t107151 = t91388 * t6390;
    let t107159 = t22827 * t1339 * t6347 * t1824 * t550;
    let t107164 = t22827 * t1339 * t1799 * t6414 * t550;
    (t107145, t107147, t107151, t107159, t107164)
}

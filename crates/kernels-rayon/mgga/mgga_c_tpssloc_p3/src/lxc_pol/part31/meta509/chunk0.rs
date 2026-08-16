//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1705/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1705(t28337: f64, t6646: f64, t22986: f64, t5527: f64, t6638: f64, t6637: f64, t23035: f64, t1484: f64, t25319: f64, t6552: f64, t5612: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28338 = t6646 * t28337;
    let t28339 = t22986 * t28338;
    let t28341 = t6638 * t5527;
    let t28342 = t6637 * t28341;
    let t28343 = t23035 * t28342;
    let t28345 = t25319 * t1484;
    let t28346 = t6637 * t28345;
    let t28347 = t6552 * t28346;
    let t28356 = t815 * t5612;
    (t28338, t28339, t28341, t28342, t28343, t28345, t28346, t28347, t28356)
}

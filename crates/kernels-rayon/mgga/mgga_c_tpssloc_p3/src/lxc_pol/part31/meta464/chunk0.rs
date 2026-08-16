//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1618/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1618(t25338: f64, t6552: f64, t4119: f64, t6554: f64, t6553: f64, t23204: f64, t7479: f64, t23164: f64, t1530: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25339 = t6552 * t25338;
    let t25341 = t6554 * t4119;
    let t25342 = t6553 * t25341;
    let t25343 = t6552 * t25342;
    let t25345 = t23204 * t7479;
    let t25346 = t23164 * t25345;
    let t25365 = t1530 * t776;
    (t25339, t25341, t25342, t25343, t25345, t25346, t25365)
}

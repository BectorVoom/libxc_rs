//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 933/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk933(t25276: f64, t25328: f64, t858: f64, t23237: f64, t7479: f64, t6552: f64, t4119: f64, t6554: f64, t6553: f64, t23204: f64, t23164: f64, t225: f64, t7511: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25329 = t25276 + t25328;
    let t25330 = t858 * t25329;
    let t25338 = t23237 * t7479;
    let t25339 = t6552 * t25338;
    let t25341 = t6554 * t4119;
    let t25342 = t6553 * t25341;
    let t25343 = t6552 * t25342;
    let t25345 = t23204 * t7479;
    let t25346 = t23164 * t25345;
    let t25348 = t7511 * t225;
    (t25329, t25330, t25339, t25341, t25343, t25346, t25348)
}

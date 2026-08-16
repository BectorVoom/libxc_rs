//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1125/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1125(t22893: f64, t7520: f64, t23164: f64, t1519: f64, t234: f64, t23204: f64, t7479: f64, t225: f64, t7511: f64, t2752: f64, t7540: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25316 = t22893 * t7520;
    let t25317 = t23164 * t25316;
    let t25319 = t234 * t1519;
    let t25345 = t23204 * t7479;
    let t25346 = t23164 * t25345;
    let t25348 = t7511 * t225;
    let t25358 = t7540 * t2752;
    (t25316, t25317, t25319, t25345, t25346, t25348, t25358)
}

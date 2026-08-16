//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 554/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk554(t2625: f64, t506: f64, t529: f64, t2531: f64, t538: f64, t560: f64, t938: f64) -> (f64, f64, f64, f64, f64) {
    let t2626 = t506 * t2625;
    let t2627 = t529 * t2626;
    let t2630 = t538 * t2531;
    let t2631 = t529 * t2630;
    let t2634 = t938 * t560;
    (t2626, t2627, t2630, t2631, t2634)
}

//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2390/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2390(t48157: f64, t60192: f64, t60194: f64, t60202: f64, t68571: f64, t68577: f64, t68580: f64, t68583: f64, t68586: f64, t68589: f64, t68592: f64, t10564: f64, t123: f64, t68521: f64) -> (f64, f64) {
    let t68594 = -0.91983333333333333333e-1_f64 * t48157 - 0.301925e0_f64 * t68571 + 0.99342e0_f64 * t60192 - 0.66228e0_f64 * t60194 - 0.33114e0_f64 * t60202 + 0.72462e1_f64 * t68577 - 0.543465e1_f64 * t68580 + 0.181155e1_f64 * t68583 + 0.181155e1_f64 * t68586 + 0.60385e0_f64 * t68589 - 0.20128333333333333333e0_f64 * t68592;
    let t68596 = t123 * t10564 * t68521;
    (t68594, t68596)
}

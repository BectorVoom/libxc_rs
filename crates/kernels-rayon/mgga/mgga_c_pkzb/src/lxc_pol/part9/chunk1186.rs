//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1186/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1186(t237: f64, t7510: f64, t732: f64, t1987: f64, t7528: f64, t1306: f64, t20636: f64, t20641: f64, t20642: f64, t20647: f64, t20649: f64, t20652: f64, t20654: f64, t20658: f64, t20662: f64, t7543: f64) -> (f64, f64, f64) {
    let t20663 = t237 * t7510;
    let t20665 = 0.17544670867903938621e1_f64 * t20663 * t732;
    let t20667 = 0.70178683471615754484e1_f64 * t1987 * t7528;
    let t20668 = 6.0_f64 * t1306 * t20642 * t7543 - t20636 + t20641 + t20647 + t20649 + t20652 + t20654 - t20658 + t20662 - t20665 + t20667;
    (t20665, t20667, t20668)
}

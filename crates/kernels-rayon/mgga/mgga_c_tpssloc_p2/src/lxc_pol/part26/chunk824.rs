//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 824/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk824(t685: f64, t9694: f64, t120: f64, t781: f64, t118: f64, t123: f64, t116: f64, t16: f64, t2397: f64, t9691: f64, t693: f64, t119: f64, t133: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9695 = t685 * t9694;
    let t9697 = t120 * t781;
    let t9698 = t118 * t9697;
    let t9700 = 1.0_f64/pow_3_2(t123);
    let t9701 = t9700 * t116;
    let t9702 = t9701 * t16;
    let t9704 = t2397 * t9691;
    let t9706 = t693 * t9694;
    let t9709 = t133 * t119 * t625;
    (t9695, t9697, t9698, t9702, t9704, t9706, t9709)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 758/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk758(t543: f64, t9768: f64, t3992: f64, t2661: f64, t212: f64, t225: f64, t596: f64, t816: f64, t3995: f64, t1408: f64, t2681: f64, t820: f64) -> (f64, f64, f64, f64, f64) {
    let t9769 = t9768 * t543;
    let t9770 = t3992 * t9769;
    let t9771 = t2661 * t9770;
    let t9775 = t816 * t596 * t212 * t225;
    let t9776 = t9775 * t3995;
    let t9779 = t820 * t1408 * t2681;
    (t9769, t9771, t9775, t9776, t9779)
}

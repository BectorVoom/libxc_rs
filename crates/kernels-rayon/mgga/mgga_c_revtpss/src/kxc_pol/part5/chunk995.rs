//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 995/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk995(t2470: f64, t2804: f64, t874: f64, t875: f64, t9288: f64, t2718: f64, t860: f64, t243: f64, t816: f64, t9707: f64, t813: f64, t2689: f64, t2694: f64) -> (f64, f64, f64, f64, f64) {
    let t10647 = t874 * t2804 * t2470;
    let t10651 = 0.30356481678079769392e-1_f64 * t874 * t875 * t9288;
    let t10661 = t2718 * t860;
    let t10671 = t9707 * t243 * t816;
    let t10673 = 0.12846167376791569079e-2_f64 * t813 * t10671;
    let t10678 = t2689 * t2694;
    (t10647, t10651, t10661, t10673, t10678)
}

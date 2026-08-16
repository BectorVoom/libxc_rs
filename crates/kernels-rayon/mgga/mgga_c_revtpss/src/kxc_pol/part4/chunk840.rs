//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 840/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk840(t1592: f64, t4786: f64, t3092: f64, t1058: f64, t1660: f64, t1053: f64, t1659: f64, t225: f64, t4743: f64, t366: f64, t1065: f64, t2857: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4787 = t1592 * t4786;
    let t4788 = t3092 * t4787;
    let t4792 = t1660 * t1058;
    let t4794 = t1659 * t1053;
    let t4797 = t4743 * t225;
    let t4798 = t4797 * t366;
    let t4801 = t1065 * t2857;
    (t4787, t4788, t4792, t4794, t4797, t4798, t4801)
}

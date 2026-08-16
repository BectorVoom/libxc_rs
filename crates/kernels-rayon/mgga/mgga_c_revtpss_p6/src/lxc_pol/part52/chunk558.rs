//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 558/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk558(t4772: f64, t996: f64, t1678: f64, t994: f64, t1668: f64, t73: f64, t3095: f64, t3092: f64, t3093: f64, t357: f64, t1592: f64, t1058: f64, t1660: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4773 = t996 * t4772;
    let t4778 = t994 * t1678;
    let t4781 = t1668 * t73;
    let t4782 = t4781 * t3095;
    let t4783 = t3092 * t4782;
    let t4786 = t3093 * t357;
    let t4787 = t1592 * t4786;
    let t4788 = t3092 * t4787;
    let t4792 = t1660 * t1058;
    (t4773, t4778, t4781, t4783, t4788, t4792)
}

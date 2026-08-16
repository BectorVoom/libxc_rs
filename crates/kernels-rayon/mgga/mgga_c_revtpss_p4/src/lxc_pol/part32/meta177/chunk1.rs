//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 823/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk823(t3095: f64, t4781: f64, t3092: f64, t3093: f64, t357: f64, t1592: f64, t1058: f64, t1660: f64, t1053: f64, t1659: f64, t225: f64, t4743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4782 = t4781 * t3095;
    let t4783 = t3092 * t4782;
    let t4786 = t3093 * t357;
    let t4787 = t1592 * t4786;
    let t4788 = t3092 * t4787;
    let t4792 = t1660 * t1058;
    let t4794 = t1659 * t1053;
    let t4797 = t4743 * t225;
    (t4782, t4783, t4786, t4787, t4788, t4792, t4794, t4797)
}

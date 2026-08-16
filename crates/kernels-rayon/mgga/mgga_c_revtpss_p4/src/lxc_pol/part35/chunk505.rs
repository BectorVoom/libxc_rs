//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 505/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk505(t378: f64, t4746: f64, t1647: f64, t1678: f64, t994: f64, t1668: f64, t73: f64, t1058: f64, t1660: f64, t1065: f64, t2857: f64, t2852: f64, t3181: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4747 = t4746 * t378;
    let t4752 = t1647 * t378;
    let t4778 = t994 * t1678;
    let t4781 = t1668 * t73;
    let t4792 = t1660 * t1058;
    let t4801 = t1065 * t2857;
    let t4806 = t3181 * t2852;
    (t4747, t4752, t4778, t4781, t4792, t4801, t4806)
}

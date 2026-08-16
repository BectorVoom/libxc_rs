//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3179/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3179(t12809: f64, t12916: f64, t17380: f64, t3568: f64, t3603: f64, t1247: f64, t1796: f64, t42994: f64, t1261: f64, t17231: f64, t3172: f64, t1250: f64) -> (f64, f64, f64, f64, f64) {
    let t58791 = t12809 * t12916 * t17380;
    let t58803 = t3603 * t3568;
    let t58824 = t1247 * t42994 * t1796;
    let t58827 = t1261 * t3172 * t17231;
    let t58831 = t1250 * t3568;
    (t58791, t58803, t58824, t58827, t58831)
}

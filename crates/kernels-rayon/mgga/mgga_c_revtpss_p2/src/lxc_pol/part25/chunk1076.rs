//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1076/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1076(t12909: f64, t3624: f64, t1250: f64, t12718: f64, t3720: f64, t126: f64, t482: f64, t828: f64, t3722: f64, t3718: f64, t1214: f64, t2251: f64) -> (f64, f64, f64, f64) {
    let t12910 = t12909 * t3624;
    let t12911 = t12718 * t1250;
    let t12912 = t3720 * t12911;
    let t12915 = t126 * t482;
    let t12916 = t828 * t12915;
    let t12917 = t12916 * t3722;
    let t12918 = t3718 * t12917;
    let t12920 = t2251 * t1214;
    (t12910, t12912, t12918, t12920)
}

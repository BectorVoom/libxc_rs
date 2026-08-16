//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1471/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1471(t2195: f64, t2289: f64, t31027: f64, t8312: f64, t31032: f64, t8316: f64, t104: f64, t2357: f64, t116: f64, t8320: f64, t10199: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31134 = 11.0_f64 / 9.0_f64 * t2289 * t2195;
    let t31135 = t31027 * t8312;
    let t31137 = t31032 * t8316;
    let t31149 = t104 * t2357;
    let t31234 = t116 * t8320;
    let t31287 = t10199 * t655;
    (t31134, t31135, t31137, t31149, t31234, t31287)
}

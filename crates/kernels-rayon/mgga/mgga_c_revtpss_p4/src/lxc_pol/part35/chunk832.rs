//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 832/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk832(t6132: f64, t698: f64, t6135: f64, t6138: f64, t300: f64, t6184: f64, t6104: f64, t914: f64, t3336: f64, t6396: f64, t964: f64, t6152: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19002 = t698 * t6132;
    let t19004 = t698 * t6135;
    let t19009 = t698 * t6138;
    let t19049 = t300 * t6184;
    let t19056 = t6104 * t914;
    let t19153 = t6396 * t3336;
    let t19156 = t6184 * t964;
    let t19173 = t6152 * t945;
    (t19002, t19004, t19009, t19049, t19056, t19153, t19156, t19173)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1109/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1109(t7036: f64, t820: f64, t844: f64, t2482: f64, t814: f64, t228: f64, t25273: f64, t25282: f64, t9802: f64, t243: f64, t7021: f64, t1941: f64, t853: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    let t92968 = t25273 * t228;
    let t92975 = t9802 * t25282;
    let t92978 = t7021 * t243;
    let t92981 = t1941 * t853;
    (t92951, t92955, t92968, t92975, t92978, t92981)
}

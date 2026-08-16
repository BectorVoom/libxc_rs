//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1828/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1828(t802: f64, t92968: f64, t25282: f64, t9802: f64, t243: f64, t7021: f64, t64: f64, t9731: f64, t2710: f64, t826: f64, t10631: f64, t10886: f64, t7028: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92969 = t92968 * t802;
    let t92975 = t9802 * t25282;
    let t92978 = t7021 * t243;
    let t92986 = t64 * t9731;
    let t92988 = t2710 * t92986 * t826;
    let t92991 = t10886 * t7028 * t10631;
    (t92969, t92975, t92978, t92986, t92988, t92991)
}

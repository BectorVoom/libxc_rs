//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 794/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk794(t10190: f64, t9397: f64, t9557: f64, t9589: f64, t2327: f64, t648: f64, t64: f64, t843: f64, t112: f64, t2289: f64, t666: f64, t2341: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10192 = t9397 + t9557 + t9589 + t10190;
    let t10194 = t648 * t2327;
    let t10199 = t64 * t843;
    let t10201 = 154.0_f64 / 27.0_f64 * t10199 * t112;
    let t10202 = t2289 * t666;
    let t10204 = t625 * t2341;
    (t10192, t10194, t10199, t10201, t10202, t10204)
}

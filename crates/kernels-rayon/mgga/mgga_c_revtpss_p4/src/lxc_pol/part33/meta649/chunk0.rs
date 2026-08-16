//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2099/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2099(t17361: f64, t7618: f64, t17289: f64, t2138: f64, t3666: f64, t8184: f64, t17451: f64, t26867: f64, t1285: f64, t97173: f64, t104646: f64, t17735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t104905 = t7618 * t17361;
    let t104916 = t17289 * t2138;
    let t104924 = t3666 * t8184;
    let t104933 = t26867 * t17451;
    let t104943 = t1285 * t97173;
    let t104946 = t17735 * t104646;
    (t104905, t104916, t104924, t104933, t104943, t104946)
}

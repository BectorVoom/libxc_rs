//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1183/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1183(t23148: f64, t30: f64, t1583: f64, t5962: f64, t25207: f64, t1544: f64, t6079: f64, t27383: f64, t23429: f64, t1468: f64, t5966: f64, t5824: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t113428 = t30 * t23148;
    let t113432 = t5962 * t1583;
    let t113433 = t25207 * t113432;
    let t113440 = t1544 * t6079;
    let t113441 = t27383 * t113440;
    let t113444 = t30 * t23429;
    let t113454 = t1468 * t5966;
    let t113461 = t5824 * t1544;
    (t113428, t113432, t113433, t113440, t113441, t113444, t113454, t113461)
}

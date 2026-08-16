//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1343/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1343(t1450: f64, t2014: f64, t532: f64, t94588: f64, t94637: f64, t94692: f64, t94744: f64, t94794: f64, t94846: f64, t94893: f64, t94934: f64, t25194: f64, t7235: f64) -> (f64, f64) {
    let t94940 = t2014 * t532 * (t94588 + t94637 + t94692 + t94744 + t94794 + t94846 + t94893 + t94934) * t1450;
    let t94942 = 6.0_f64 * t7235 * t25194;
    (t94940, t94942)
}

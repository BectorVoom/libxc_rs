//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2026/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2026(t25418: f64, t689: f64, t93302: f64, t25331: f64, t25365: f64, t25325: f64, t686: f64, t72: f64, t25387: f64, t25372: f64, t93301: f64, t25386: f64, t93280: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93303 = t25418 * t689;
    let t93304 = t93302 * t93303;
    let t93306 = t25365 * t25331;
    let t93311 = t25325 * t72 * t686;
    let t93312 = t25387 * t93311;
    let t93314 = t25372 * t93301;
    let t93315 = t93314 * t93303;
    let t93317 = t25386 * t93280;
    (t93304, t93306, t93311, t93312, t93315, t93317)
}

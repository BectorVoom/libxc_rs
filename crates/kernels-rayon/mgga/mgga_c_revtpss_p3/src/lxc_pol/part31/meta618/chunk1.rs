//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2066/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2066(t2435: f64, t27195: f64, t1955: f64, t27198: f64, t2769: f64, t2470: f64, t27278: f64, t7064: f64, t10073: f64, t25402: f64, t7056: f64, t7759: f64) -> (f64, f64, f64, f64, f64) {
    let t99188 = t2435 * t27195;
    let t99191 = t1955 * t27198 * t2769;
    let t99201 = t27278 * t2470;
    let t99202 = t7064 * t99201;
    let t99206 = t10073 * t7056 * t25402 * t7759;
    (t99188, t99191, t99201, t99202, t99206)
}

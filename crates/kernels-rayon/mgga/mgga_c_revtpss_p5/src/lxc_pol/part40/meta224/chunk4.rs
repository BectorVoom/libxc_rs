//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 887/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk887(t1469: f64, t3367: f64, t606: f64, t1120: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t5051 = t3367 * t1469;
    let t5052 = t5051 * t606;
    let t5053 = t1120 * t5052;
    let t5054 = t128 * t5053;
    (t5051, t5052, t5053, t5054)
}

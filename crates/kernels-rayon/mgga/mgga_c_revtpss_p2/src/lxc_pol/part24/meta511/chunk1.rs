//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1527/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1527(t11256: f64, t23642: f64, t3172: f64, t23811: f64, t300: f64, t1063: f64, t23470: f64, t247: f64, t42534: f64, t20050: f64, t4834: f64, t23843: f64) -> (f64, f64, f64, f64, f64) {
    let t78676 = t11256 * t3172 * t23642;
    let t78704 = t300 * t23811;
    let t78750 = t1063 * t247 * t42534 * t23470;
    let t78756 = t4834 * t20050;
    let t78763 = t1063 * t3172 * t23843;
    (t78676, t78704, t78750, t78756, t78763)
}

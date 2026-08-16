//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1121/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1121(t119836: f64, t119893: f64, t1032: f64, t2735: f64, t119867: f64, t233: f64, t240: f64, t31838: f64, t31840: f64, t845: f64, t31834: f64, t846: f64) -> (f64, f64, f64, f64, f64) {
    let t119894 = t119836 * t119893;
    let t119900 = t2735 * t1032;
    let t119903 = t119900 * t233 * t240 * t119867;
    let t119912 = t31838 * t845 * t31840;
    let t119913 = 0.34708173928447610098e-2_f64 * t119912;
    let t119914 = t31834 * t846;
    (t119894, t119900, t119903, t119913, t119914)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 592/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk592(t6022: f64, t827: f64, t828: f64, t5962: f64, t855: f64, t1544: f64, t231: f64) -> (f64, f64, f64) {
    let t6024 = t827 * t828 * t6022;
    let t6030 = t855 * t828 * t5962;
    let t6035 = t231 * t1544;
    (t6024, t6030, t6035)
}

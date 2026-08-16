//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 736/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk736(t72: f64, t752: f64, t757: f64, t2492: f64, t2596: f64, t745: f64) -> (f64, f64, f64, f64) {
    let t2622 = t752 * t72;
    let t2623 = t2622 * t757;
    let t2624 = 0.36622894612013090108e-3_f64 * t2623;
    let t2626 = t2596 * t2492 * t745;
    (t2622, t2623, t2624, t2626)
}

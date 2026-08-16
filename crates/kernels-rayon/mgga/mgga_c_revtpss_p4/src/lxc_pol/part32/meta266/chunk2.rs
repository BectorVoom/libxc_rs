//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1123/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1123(t670: f64, t7553: f64, t117: f64, t7373: f64, t1459: f64, t1461: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t7547: f64, t38: f64, t4173: f64) -> (f64, f64, f64, f64) {
    let t7554 = t7553 * t670;
    let t7557 = t117 * t7373;
    let t7560 = 3.0_f64 * t1459 * t2115 + 3.0_f64 * t1461 * t2113 + 6.0_f64 * t572 * t7554 + 3.0_f64 * t572 * t7557 + t573 * t7547;
    let t7702 = t4173 * t38;
    (t7554, t7557, t7560, t7702)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1653/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1653(t14365: f64, t25207: f64, t605: f64, t775: f64, t2430: f64, t30: f64, t1946: f64, t2684: f64, t7043: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64) {
    let t25208 = t25207 * t14365;
    let t25211 = t605 * t775;
    let t25215 = t30 * t2430;
    let t25219 = t1946 * t2684;
    let t25222 = t820 * t7043 * t843;
    (t25208, t25211, t25215, t25219, t25222)
}

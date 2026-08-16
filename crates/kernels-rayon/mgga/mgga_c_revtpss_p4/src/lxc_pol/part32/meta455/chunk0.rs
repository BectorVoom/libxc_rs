//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1654/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1654(t159: f64, t2698: f64, t218: f64, t816: f64, t228: f64, t7021: f64, t802: f64, t7043: f64, t826: f64, t2736: f64, t2453: f64, t7057: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25273 = t2698 * t159;
    let t25275 = t25273 * t218 * t816;
    let t25277 = t7021 * t228;
    let t25278 = t25277 * t802;
    let t25279 = 7.0_f64 / 72.0_f64 * t25278;
    let t25282 = t7043 * t826;
    let t25283 = t2736 * t25282;
    let t25299 = t2453 * t7057;
    (t25273, t25275, t25277, t25279, t25282, t25283, t25299)
}

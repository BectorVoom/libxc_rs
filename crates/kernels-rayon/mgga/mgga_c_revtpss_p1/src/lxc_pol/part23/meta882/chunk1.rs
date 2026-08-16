//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2793/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2793(t2782: f64, t47371: f64, t75047: f64, t1398: f64, t6862: f64, t10022: f64, t22315: f64, t46457: f64, t136: f64, t2457: f64, t47429: f64, t10014: f64, t22332: f64) -> (f64, f64, f64, f64, f64) {
    let t75049 = t2782 * t47371 * t75047;
    let t75051 = t6862 * t1398;
    let t75053 = t2782 * t10022 * t75051;
    let t75060 = t46457 * t22315;
    let t75068 = t47429 * t6862 * t136 * t2457;
    let t75071 = t10014 * t22332;
    (t75049, t75053, t75060, t75068, t75071)
}

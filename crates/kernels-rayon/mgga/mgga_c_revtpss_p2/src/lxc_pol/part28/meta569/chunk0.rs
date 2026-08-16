//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2029/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2029(t25374: f64, t93341: f64, t25378: f64, t11050: f64, t25399: f64, t11007: f64, t1955: f64, t7056: f64, t93320: f64, t25387: f64, t93330: f64, t25410: f64, t93189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t93342 = t93341 * t25374;
    let t93343 = t93342 * t25378;
    let t93346 = t25399 * t11050;
    let t93349 = t1955 * t7056 * t11007;
    let t93364 = t93320 * t25374;
    let t93365 = t93364 * t25378;
    let t93369 = t25387 * t93330;
    let t93371 = t93189 * t25410;
    (t93342, t93343, t93346, t93349, t93364, t93365, t93369, t93371)
}

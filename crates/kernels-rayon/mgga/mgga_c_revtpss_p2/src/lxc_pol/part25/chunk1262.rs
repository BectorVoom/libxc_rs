//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1262/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1262(t11050: f64, t25399: f64, t11007: f64, t1955: f64, t7056: f64, t231: f64, t2771: f64, t836: f64, t10867: f64, t867: f64, t25374: f64, t93320: f64) -> (f64, f64, f64, f64, f64) {
    let t93346 = t25399 * t11050;
    let t93349 = t1955 * t7056 * t11007;
    let t93351 = t2771 * t836 * t231;
    let t93355 = t867 * t10867;
    let t93364 = t93320 * t25374;
    (t93346, t93349, t93351, t93355, t93364)
}

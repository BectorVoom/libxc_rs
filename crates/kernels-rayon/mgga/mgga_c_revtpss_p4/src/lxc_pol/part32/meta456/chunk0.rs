//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1656/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1656(t25304: f64, t7057: f64, t1032: f64, t860: f64, t867: f64, t786: f64, t11007: f64, t233: f64) -> (f64, f64, f64, f64, f64) {
    let t25305 = t25304 * t7057;
    let t25308 = t860 * t1032;
    let t25309 = t25308 * t867;
    let t25310 = t786 * t25309;
    let t25317 = t11007 * t233;
    (t25305, t25308, t25309, t25310, t25317)
}

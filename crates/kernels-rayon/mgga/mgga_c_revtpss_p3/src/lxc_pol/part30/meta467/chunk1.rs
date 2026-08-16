//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1773/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1773(t25296: f64, t7058: f64, t2453: f64, t7057: f64, t136: f64, t1958: f64, t2457: f64) -> (f64, f64, f64, f64) {
    let t25297 = t7058 * t25296;
    let t25299 = t2453 * t7057;
    let t25300 = t1958 * t136;
    let t25301 = t25300 * t2457;
    (t25297, t25299, t25300, t25301)
}

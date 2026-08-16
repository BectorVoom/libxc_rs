//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 745/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk745(t555: f64, t9990: f64, t1432: f64, t1433: f64, t9288: f64, t225: f64, t9646: f64, t1428: f64, t22: f64, t2452: f64) -> (f64, f64, f64, f64, f64) {
    let t10090 = t9990 * t555;
    let t10102 = 0.30356481678079769392e-1_f64 * t1432 * t1433 * t9288;
    let t10111 = t9646 * t225;
    let t10114 = 0.19637199382202157274e-3_f64 * t10111 * t1428 * t22;
    let t10115 = t22 * t2452;
    (t10090, t10102, t10111, t10114, t10115)
}

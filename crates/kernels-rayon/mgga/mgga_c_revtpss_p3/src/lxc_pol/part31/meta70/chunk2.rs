//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 452/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk452(t546: f64, t550: f64, t808: f64, t807: f64, t547: f64, t786: f64, t814: f64, t816: f64, t544: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1376 = t546 * t808 * t550;
    let t1378 = 0.71456696863449561619e-5_f64 * t807 * t1376;
    let t1379 = t786 * t547;
    let t1380 = t814 * t550;
    let t1381 = t1380 * t816;
    let t1383 = 0.12705000702321332056e-4_f64 * t1379 * t1381;
    let t1384 = t544 * t544;
    let t1385 = 1.0_f64 / t1384;
    (t1376, t1378, t1379, t1381, t1383, t1384, t1385)
}

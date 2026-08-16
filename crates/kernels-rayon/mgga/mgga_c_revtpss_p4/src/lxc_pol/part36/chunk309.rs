//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 309/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk309(t1362: f64, t1364: f64, t535: f64, t795: f64, t159: f64, t540: f64, t216: f64, t546: f64, t550: f64, t808: f64, t807: f64, t547: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1366 = 0.9757440539382783019e-2_f64 * t1362 * t1364;
    let t1368 = 7.0_f64 / 288.0_f64 * t795 * t535;
    let t1369 = t159 * t540;
    let t1370 = t216 * t1369;
    let t1376 = t546 * t808 * t550;
    let t1378 = 0.71456696863449561619e-5_f64 * t807 * t1376;
    let t1379 = t786 * t547;
    (t1366, t1368, t1369, t1370, t1376, t1378, t1379)
}

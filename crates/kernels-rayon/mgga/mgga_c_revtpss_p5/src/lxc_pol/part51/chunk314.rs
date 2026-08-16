//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 314/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk314(t1426: f64, t225: f64, t545: f64, t555: f64, t869: f64, t689: f64, t546: f64, t786: f64, t72: f64, t686: f64, t1385: f64, t1399: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1427 = t225 * t1426;
    let t1428 = t545 * t555;
    let t1429 = t869 * t1428;
    let t1431 = 0.54878743191129263322e-2_f64 * t689 * t1429;
    let t1432 = t786 * t546;
    let t1433 = t555 * t72;
    let t1436 = 0.9757440539382783019e-2_f64 * t1432 * t1433 * t686;
    let t1437 = t1385 * t555;
    let t1438 = t1437 * t1399;
    (t1427, t1428, t1429, t1431, t1432, t1433, t1436, t1437, t1438)
}

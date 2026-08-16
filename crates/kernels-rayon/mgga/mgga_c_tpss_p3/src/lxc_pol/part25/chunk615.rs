//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 615/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk615(t3629: f64, t3630: f64, t3628: f64, t1381: f64, t2169: f64, t2177: f64, t2175: f64, t2224: f64, t2281: f64, t2285: f64, t3546: f64, t3547: f64, t3559: f64, t3562: f64, t3563: f64, t3568: f64, t3571: f64, t3574: f64, t3592: f64) -> (f64, f64, f64, f64) {
    let t3631 = t3629 * t3630;
    let t3632 = t3628 * t3631;
    let t3635 = t2169 * t1381;
    let t3637 = t3629 * t2177;
    let t3638 = t2175 * t3637;
    let t3641 = t3546 + t3547 - t3559 - t3562 + t2224 - t2285 - t3563 + t3568 - t2281 + t3571 + t3574 + t3592;
    (t3632, t3635, t3638, t3641)
}

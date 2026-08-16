//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1229/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1229(t1395: f64, t5831: f64, t5572: f64, t1805: f64, t3721: f64, t18770: f64, t19762: f64, t2157: f64, t19769: f64, t1378: f64, t226: f64, t5577: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20470 = t5831 * t1395;
    let t20471 = t5572 * t20470;
    let t20474 = t1805 * t3721;
    let t20475 = t5572 * t20474;
    let t20479 = t18770 * t19762;
    let t20482 = t2157 * t1805;
    let t20483 = t20482 * t19769;
    let t20487 = t5831 * t1378 * t226;
    let t20488 = t5577 * t20487;
    (t20471, t20475, t20479, t20482, t20483, t20488)
}

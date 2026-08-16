//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1042/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1042(t4799: f64, t818: f64, t2406: f64, t246: f64, t4715: f64, t1378: f64, t1388: f64, t4758: f64, t4778: f64, t768: f64, t10845: f64, t10884: f64, t1379: f64, t14179: f64, t14210: f64, t14298: f64, t14349: f64, t220: f64, t229: f64, t2415: f64, t339: f64, t3630: f64, t3665: f64, t3703: f64, t3704: f64, t3713: f64, t3716: f64, t4716: f64, t4759: f64, t4764: f64, t783: f64, t813: f64, t8361: f64) -> (f64, f64) {
    let t14371 = t4799 * t818;
    let t14372 = t2406 * t14371;
    let t14375 = t246 * t4715;
    let t14388 = t1388 * t1378;
    let t14401 = t246 * t4758;
    let t14418 = t768 * t4778;
    let t14423 = -6.0_f64 * t10845 * t14210 * t14375 - 2.0_f64 * t10884 * t1379 * t339 + 4.0_f64 * t14179 * t3703 * t3704 - t14298 * t339 * t813 + t14349 * t220 * t229 + 6.0_f64 * t14375 * t3630 * t3703 - t14375 * t3713 * t783 + 4.0_f64 * t14388 * t3630 * t3703 - 2.0_f64 * t14388 * t3713 * t783 + 2.0_f64 * t14401 * t3630 * t3703 - t14401 * t3713 * t783 - t14418 * t339 * t783 - t2415 * t339 * t4759 - t2415 * t339 * t4764 - 2.0_f64 * t339 * t3665 * t3716 + 2.0_f64 * t339 * t4716 * t8361 - 2.0_f64 * t3665 * t3704 * t3713;
    (t14372, t14423)
}

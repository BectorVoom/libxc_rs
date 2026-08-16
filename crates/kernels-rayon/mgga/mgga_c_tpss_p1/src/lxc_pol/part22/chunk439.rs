//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 439/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk439(t1569: f64, t339: f64, t454: f64, t1128: f64, t1501: f64, t242: f64, t1097: f64, t1098: f64, t1111: f64, t1122: f64, t1125: f64, t1554: f64, t1558: f64, t1564: f64, t444: f64, t463: f64) -> (f64, f64) {
    let t1571 = t339 * t454 * t1569;
    let t1574 = t1128 * t1501;
    let t1575 = t242 * t1574;
    let t1578 = -t1554 * t444 / 36.0_f64 + t1097 - t1098 * t1558 / 288.0_f64 + t1111 * t1564 / 3072.0_f64 - t1571 * t463 / 576.0_f64 + t1122 - t1125 * t1575 / 4608.0_f64;
    (t1571, t1578)
}

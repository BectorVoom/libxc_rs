//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 961/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk961(t11315: f64, t923: f64, t11156: f64, t2908: f64, t141: f64, t11165: f64, t930: f64, t2912: f64, t698: f64, t11151: f64, t11160: f64, t11132: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11316 = t923 * t11315;
    let t11318 = t2908 * t11156;
    let t11319 = t141 * t11318;
    let t11321 = t930 * t11165;
    let t11322 = t141 * t11321;
    let t11326 = t698 * t2912;
    let t11328 = t2908 * t11151;
    let t11329 = t141 * t11328;
    let t11331 = t930 * t11160;
    let t11332 = t141 * t11331;
    let t11334 = 0.93011851851851851854e0_f64 * t11132;
    (t11316, t11319, t11322, t11326, t11329, t11332, t11334)
}

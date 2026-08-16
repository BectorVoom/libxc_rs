//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1013/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1013(t35755: f64, t30219: f64, t8469: f64, t1562: f64, t31824: f64, t1449: f64, t30148: f64, t30159: f64, t7586: f64, t1541: f64, t31611: f64, t8473: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35756 = 0.17149607247227894789e-1_f64 * t35755;
    let t35774 = t30219 * t8469;
    let t35775 = 0.31448092289604152068e-2_f64 * t35774;
    let t35784 = t31824 * t1562;
    let t35785 = 0.34299214494455789578e-2_f64 * t35784;
    let t35788 = t30159 * t7586 * t30148 * t1449;
    let t35789 = 0.12579236915841660827e-2_f64 * t35788;
    let t35790 = t31611 * t1541;
    let t35794 = t30219 * t8473;
    (t35756, t35775, t35785, t35789, t35790, t35794)
}

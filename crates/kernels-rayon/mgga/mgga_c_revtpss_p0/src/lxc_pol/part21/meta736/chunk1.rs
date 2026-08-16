//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2587/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2587(t10008: f64, t1358: f64, t212: f64, t689: f64, t1359: f64, t39501: f64, t10115: f64, t555: f64, t1445: f64, t10165: f64, t9664: f64, t1427: f64, t1444: f64, t22: f64, t9647: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47558 = t689 * t212 * t10008 * t1358;
    let t47561 = 0.56911289235245161963e-1_f64 * t39501 * t1359;
    let t47567 = t10115 * t555;
    let t47568 = t47567 * t1445;
    let t47570 = t10165 * t9664;
    let t47574 = t9647 * t1427 * t22 * t1444;
    (t47558, t47561, t47567, t47568, t47570, t47574)
}

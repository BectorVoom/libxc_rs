//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1784/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1784(t10115: f64, t555: f64, t1445: f64, t10165: f64, t9664: f64, t1427: f64, t1444: f64, t22: f64, t9647: f64, t123: f64, t2434: f64, t4077: f64, t9680: f64) -> (f64, f64, f64, f64) {
    let t47567 = t10115 * t555;
    let t47568 = t47567 * t1445;
    let t47570 = t10165 * t9664;
    let t47574 = t9647 * t1427 * t22 * t1444;
    let t47580 = t9680 * t123 * t2434 * t4077;
    (t47568, t47570, t47574, t47580)
}

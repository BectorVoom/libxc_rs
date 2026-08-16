//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1001/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1001(t1358: f64, t9634: f64, t689: f64, t3896: f64, t9303: f64, t1419: f64, t785: f64, t2439: f64, t784: f64, t209: f64) -> (f64, f64, f64, f64) {
    let t9635 = t9634 * t1358;
    let t9636 = t689 * t9635;
    let t9639 = 0.26019841438354088051e-2_f64 * t9303 * t3896;
    let t9640 = t785 * t1419;
    let t9641 = t9640 * t1358;
    let t9642 = t2439 * t9641;
    let t9644 = t784 * t784;
    let t9645 = 1.0_f64 / t9644;
    let t9646 = t209 * t9645;
    (t9636, t9639, t9642, t9646)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 964/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk964(t1333: f64, t3860: f64, t30: f64, t513: f64, t33: f64, t516: f64, t2435: f64, t3900: f64, t3896: f64, t9303: f64, t1419: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9597 = t3860 * t1333;
    let t9603 = t30 * t30;
    let t9605 = 1.0_f64 / t513 / t9603;
    let t9615 = t33 * t33;
    let t9617 = 1.0_f64 / t516 / t9615;
    let t9632 = t2435 * t3900;
    let t9639 = 0.26019841438354088051e-2_f64 * t9303 * t3896;
    let t9640 = t785 * t1419;
    (t9597, t9605, t9617, t9632, t9639, t9640)
}

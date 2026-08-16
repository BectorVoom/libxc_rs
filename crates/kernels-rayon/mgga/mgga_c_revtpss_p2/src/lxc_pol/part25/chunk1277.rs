//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1277/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1277(t25490: f64, t3215: f64, t11951: f64, t7117: f64, t11643: f64, t25522: f64, t12009: f64, t25505: f64, t25531: f64, t800: f64, t25539: f64, t3244: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93683 = t25490 * t3215;
    let t93685 = t7117 * t11951;
    let t93687 = t25522 * t11643;
    let t93689 = t25505 * t12009;
    let t93691 = t25531 * t800;
    let t93694 = t25539 * t3244;
    (t93683, t93685, t93687, t93689, t93691, t93694)
}

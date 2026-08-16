//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2039/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2039(t11951: f64, t7117: f64, t11643: f64, t25522: f64, t12009: f64, t25505: f64, t25531: f64, t800: f64, t25539: f64, t3244: f64, t11880: f64, t7111: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93685 = t7117 * t11951;
    let t93687 = t25522 * t11643;
    let t93689 = t25505 * t12009;
    let t93691 = t25531 * t800;
    let t93694 = t25539 * t3244;
    let t93696 = t7111 * t11880;
    (t93685, t93687, t93689, t93691, t93694, t93696)
}

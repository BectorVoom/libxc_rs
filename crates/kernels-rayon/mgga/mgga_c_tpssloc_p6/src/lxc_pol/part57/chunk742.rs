//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 742/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk742(t25144: f64, t1509: f64, t2047: f64, t7823: f64, t814: f64, t25293: f64, t25317: f64, t225: f64, t7824: f64, t25211: f64, t7815: f64, t25346: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26646 = 7.0_f64 / 1152.0_f64 * t25144;
    let t26656 = t2047 * t1509;
    let t26661 = t814 * t7823;
    let t26667 = 0.38381794893125283518e-1_f64 * t25293;
    let t26673 = 0.16449340668482264365e-1_f64 * t25317;
    let t26700 = t7824 * t225;
    let t26712 = 0.38381794893125283518e-1_f64 * t25211;
    let t26713 = t7815 * t225;
    let t26726 = 0.16449340668482264365e-1_f64 * t25346;
    (t26646, t26656, t26661, t26667, t26673, t26700, t26712, t26713, t26726)
}

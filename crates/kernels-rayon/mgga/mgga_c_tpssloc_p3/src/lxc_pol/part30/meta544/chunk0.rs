//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1895/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1895(t25049: f64, t25277: f64, t25077: f64, t25080: f64, t25140: f64, t25144: f64, t25293: f64, t25317: f64, t25211: f64, t25346: f64, t26198: f64, t26200: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26591 = 0.38381794893125283518e-1_f64 * t25049;
    let t26613 = 0.38381794893125283518e-1_f64 * t25277;
    let t26619 = 7.0_f64 / 288.0_f64 * t25077;
    let t26621 = 7.0_f64 / 1152.0_f64 * t25080;
    let t26644 = 7.0_f64 / 72.0_f64 * t25140;
    let t26646 = 7.0_f64 / 1152.0_f64 * t25144;
    let t26667 = 0.38381794893125283518e-1_f64 * t25293;
    let t26673 = 0.16449340668482264365e-1_f64 * t25317;
    let t26712 = 0.38381794893125283518e-1_f64 * t25211;
    let t26726 = 0.16449340668482264365e-1_f64 * t25346;
    let t26988 = 0.16449340668482264365e-1_f64 * t26198;
    let t26993 = 0.38381794893125283518e-1_f64 * t26200;
    (t26591, t26613, t26619, t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993)
}

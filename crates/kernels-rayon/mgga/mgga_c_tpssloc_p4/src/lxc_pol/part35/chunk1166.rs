//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1166/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1166(t25080: f64, t25140: f64, t25144: f64, t25293: f64, t25317: f64, t25211: f64, t25346: f64, t26198: f64, t26200: f64, t26231: f64, t26251: f64, t26255: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26621 = 7.0_f64 / 1152.0_f64 * t25080;
    let t26644 = 7.0_f64 / 72.0_f64 * t25140;
    let t26646 = 7.0_f64 / 1152.0_f64 * t25144;
    let t26667 = 0.38381794893125283518e-1_f64 * t25293;
    let t26673 = 0.16449340668482264365e-1_f64 * t25317;
    let t26712 = 0.38381794893125283518e-1_f64 * t25211;
    let t26726 = 0.16449340668482264365e-1_f64 * t25346;
    let t26988 = 0.16449340668482264365e-1_f64 * t26198;
    let t26993 = 0.38381794893125283518e-1_f64 * t26200;
    let t27012 = 7.0_f64 / 1152.0_f64 * t26231;
    let t27019 = 7.0_f64 / 1152.0_f64 * t26251;
    let t27022 = 7.0_f64 / 288.0_f64 * t26255;
    (t26621, t26644, t26646, t26667, t26673, t26712, t26726, t26988, t26993, t27012, t27019, t27022)
}

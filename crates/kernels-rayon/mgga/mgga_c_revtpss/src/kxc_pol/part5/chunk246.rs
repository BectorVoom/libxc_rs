//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 246/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk246(t234: f64, t243: f64, t808: f64, t807: f64, t236: f64, t786: f64, t240: f64, t27: f64) -> (f64, f64, f64, f64) {
    let t810 = t234 * t808 * t243;
    let t812 = 0.71456696863449561619e-5_f64 * t807 * t810;
    let t813 = t786 * t236;
    let t814 = t27 * t240;
    (t810, t812, t813, t814)
}

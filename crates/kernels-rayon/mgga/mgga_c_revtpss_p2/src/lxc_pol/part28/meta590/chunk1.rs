//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2061/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2061(t94589: f64, t94590: f64, t7289: f64, t94377: f64, t122: f64, t72: f64, t7274: f64, t3916: f64, t25895: f64, t7285: f64, t9288: f64, t7284: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94591 = t94589 * t94590;
    let t94593 = t7289 * t94377;
    let t94596 = t7274 * t72 * t122;
    let t94597 = t94596 * t3916;
    let t94598 = t25895 * t94597;
    let t94600 = t7285 * t9288;
    let t94602 = 0.22487184191643109717e-1_f64 * t7284 * t94600;
    (t94591, t94593, t94596, t94597, t94598, t94600, t94602)
}

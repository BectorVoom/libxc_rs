//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1362/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1362(t135: f64, t24847: f64, t7284: f64, t24853: f64, t1090: f64, t24821: f64, t24574: f64, t24778: f64, t24762: f64, t1089: f64, t1235: f64, t7327: f64) -> (f64, f64, f64, f64, f64) {
    let t86094 = t24847 * t135 * t7284;
    let t86095 = t86094 * t24853;
    let t86102 = t24821 * t1090;
    let t86106 = t24574 * t24778;
    let t86113 = t24574 * t24762;
    let t86116 = t7327 * t1235 * t1089;
    (t86095, t86102, t86106, t86113, t86116)
}

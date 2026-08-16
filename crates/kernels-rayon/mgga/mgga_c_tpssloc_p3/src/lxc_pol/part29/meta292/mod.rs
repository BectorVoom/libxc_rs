//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1320;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1321;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta292(t685: f64, t9694: f64, t120: f64, t781: f64, t118: f64, t123: f64, t116: f64, t16: f64, t2397: f64, t9691: f64, t693: f64, t119: f64, t133: f64, t625: f64, t9689: f64, t9692: f64, t739: f64, t746: f64, t761: f64, t172: f64, t2448: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9695, t9697, t9698, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1320(t685, t9694, t120, t781, t118, t123, t116, t16, t2397, t9691, t693, t119, t133, t625);
        let (t9711, t9713, t9715, t9716) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1321(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709, t739, t746, t761, t172, t2448);
    (t9695, t9697, t9698, t9702, t9704, t9706, t9709, t9711, t9713, t9715, t9716)
}

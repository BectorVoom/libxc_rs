//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1156;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1157;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta282(t2617: f64, t2638: f64, t116: f64, t126: f64, t136: f64, t16: f64, t2386: f64, t625: f64, t2385: f64, t686: f64, t781: f64, t685: f64, t120: f64, t118: f64, t123: f64, t2397: f64, t693: f64, t119: f64, t133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9674, t9689, t9691, t9692, t9694, t9695) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1156(t2617, t2638, t116, t126, t136, t16, t2386, t625, t2385, t686, t781, t685);
        let (t9697, t9698, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1157(t120, t781, t118, t123, t116, t16, t2397, t9691, t693, t9694, t119, t133, t625);
    (t9674, t9689, t9692, t9695, t9697, t9698, t9702, t9704, t9706, t9709)
}

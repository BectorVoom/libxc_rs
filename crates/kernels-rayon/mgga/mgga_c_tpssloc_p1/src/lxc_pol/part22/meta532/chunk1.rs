//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2006/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2006(t591: f64, t9701: f64, t2397: f64, t39277: f64, t39280: f64, t693: f64, t119: f64, t133: f64, t240: f64, t39273: f64, t39275: f64, t39278: f64, t39281: f64, t39284: f64, t39289: f64) -> (f64, f64, f64, f64, f64) {
    let t39291 = t9701 * t591;
    let t39293 = t2397 * t39277;
    let t39295 = t693 * t39280;
    let t39298 = t133 * t119 * t240;
    let t39300 = -0.28769444444444444444e1_f64 * t39273 + 0.27618666666666666667e2_f64 * t39275 - 0.10229135802469135803e2_f64 * t39278 + 0.89504938271604938273e1_f64 * t39281 + 0.31310740740740740741e1_f64 * t39284 + 0.366775e-1_f64 * t39289 - 0.58684e0_f64 * t39291 + 0.65204444444444444445e0_f64 * t39293 + 0.5705388888888888889e0_f64 * t39295 + 0.13490888888888888889e1_f64 * t39298;
    (t39291, t39293, t39295, t39298, t39300)
}

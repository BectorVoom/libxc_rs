//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3094/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3094(t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63841: f64, t63843: f64, t63845: f64) -> f64 {
    let t64132 = -0.68863333333333333333e0_f64 * t63291 + 0.20659e1_f64 * t63296 + 0.103295e1_f64 * t63300 + 0.309885e1_f64 * t63304 + 0.22954444444444444444e0_f64 * t63306 - 0.38257407407407407407e0_f64 * t63308 - 0.68863333333333333334e0_f64 * t63313 - 0.34431666666666666667e0_f64 * t63317 + 0.91817777777777777776e0_f64 * t50826 - 0.34431666666666666666e0_f64 * t50828 - 0.10712074074074074074e1_f64 * t50834 - 0.61745185185185185186e-1_f64 * t63841 - 0.27785333333333333334e0_f64 * t63843 + 0.46308888888888888889e-1_f64 * t63845;
    t64132
}

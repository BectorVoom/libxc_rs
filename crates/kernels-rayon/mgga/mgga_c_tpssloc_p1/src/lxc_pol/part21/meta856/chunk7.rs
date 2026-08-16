//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3103/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3103(t4819: f64, t3331: f64, t6031: f64, t50826: f64, t50828: f64, t50834: f64, t63291: f64, t63296: f64, t63300: f64, t63304: f64, t63306: f64, t63308: f64, t63313: f64, t63317: f64, t63841: f64, t63843: f64, t63845: f64) -> (f64, f64, f64) {
    let t64261 = t4819 * t4819;
    let t64292 = t6031 * t3331;
    let t64309 = -0.40256666666666666667e0_f64 * t63291 + 0.12077e1_f64 * t63296 + 0.60385e0_f64 * t63300 + 0.181155e1_f64 * t63304 + 0.13418888888888888889e0_f64 * t63306 - 0.22364814814814814814e0_f64 * t63308 - 0.40256666666666666666e0_f64 * t63313 - 0.20128333333333333333e0_f64 * t63317 + 0.53675555555555555558e0_f64 * t50826 - 0.20128333333333333334e0_f64 * t50828 - 0.62621481481481481484e0_f64 * t50834 - 0.49057777777777777778e-1_f64 * t63841 - 0.22076e0_f64 * t63843 + 0.36793333333333333334e-1_f64 * t63845;
    (t64261, t64292, t64309)
}

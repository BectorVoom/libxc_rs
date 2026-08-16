//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2909/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2909(t42212: f64, t42213: f64, t59680: f64, t59684: f64, t59688: f64, t59692: f64, t59694: f64, t60223: f64, t60226: f64, t60229: f64, t60232: f64, t60235: f64, t60238: f64, t60240: f64) -> f64 {
    let t60649 = 0.34431666666666666666e0_f64 * t59680 - 0.516475e0_f64 * t59684 + 0.45908888888888888889e0_f64 * t59688 + 0.20659e1_f64 * t59692 - 0.22954444444444444444e0_f64 * t59694 - 0.69463333333333333334e-1_f64 * t60223 - 0.34731666666666666667e-1_f64 * t60226 - 0.46308888888888888889e-1_f64 * t60229 - 0.125034e1_f64 * t60232 - 0.62517e0_f64 * t60235 + t42212 + t42213 + 0.10589175e2_f64 * t60238 - 0.6311625e0_f64 * t60240;
    t60649
}

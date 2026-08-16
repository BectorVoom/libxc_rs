//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3103/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3103<F: Float>(t4819: F, t3331: F, t6031: F, t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63841: F, t63843: F, t63845: F) -> (F, F, F) {
    let t64261 = t4819 * t4819;
    let t64292 = t6031 * t3331;
    let t64309 = -F::cast_from(0.40256666666666666667e0_f64) * t63291 + F::cast_from(0.12077e1_f64) * t63296 + F::cast_from(0.60385e0_f64) * t63300 + F::cast_from(0.181155e1_f64) * t63304 + F::cast_from(0.13418888888888888889e0_f64) * t63306 - F::cast_from(0.22364814814814814814e0_f64) * t63308 - F::cast_from(0.40256666666666666666e0_f64) * t63313 - F::cast_from(0.20128333333333333333e0_f64) * t63317 + F::cast_from(0.53675555555555555558e0_f64) * t50826 - F::cast_from(0.20128333333333333334e0_f64) * t50828 - F::cast_from(0.62621481481481481484e0_f64) * t50834 - F::cast_from(0.49057777777777777778e-1_f64) * t63841 - F::cast_from(0.22076e0_f64) * t63843 + F::cast_from(0.36793333333333333334e-1_f64) * t63845;
    (t64261, t64292, t64309)
}

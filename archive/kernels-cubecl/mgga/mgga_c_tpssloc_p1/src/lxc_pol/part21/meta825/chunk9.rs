//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2909/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2909<F: Float>(t42212: F, t42213: F, t59680: F, t59684: F, t59688: F, t59692: F, t59694: F, t60223: F, t60226: F, t60229: F, t60232: F, t60235: F, t60238: F, t60240: F) -> F {
    let t60649 = F::cast_from(0.34431666666666666666e0_f64) * t59680 - F::cast_from(0.516475e0_f64) * t59684 + F::cast_from(0.45908888888888888889e0_f64) * t59688 + F::cast_from(0.20659e1_f64) * t59692 - F::cast_from(0.22954444444444444444e0_f64) * t59694 - F::cast_from(0.69463333333333333334e-1_f64) * t60223 - F::cast_from(0.34731666666666666667e-1_f64) * t60226 - F::cast_from(0.46308888888888888889e-1_f64) * t60229 - F::cast_from(0.125034e1_f64) * t60232 - F::cast_from(0.62517e0_f64) * t60235 + t42212 + t42213 + F::cast_from(0.10589175e2_f64) * t60238 - F::cast_from(0.6311625e0_f64) * t60240;
    t60649
}

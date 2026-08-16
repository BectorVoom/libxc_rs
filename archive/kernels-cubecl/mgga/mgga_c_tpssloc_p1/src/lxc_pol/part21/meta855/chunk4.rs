//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3094/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3094<F: Float>(t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63841: F, t63843: F, t63845: F) -> F {
    let t64132 = -F::cast_from(0.68863333333333333333e0_f64) * t63291 + F::cast_from(0.20659e1_f64) * t63296 + F::cast_from(0.103295e1_f64) * t63300 + F::cast_from(0.309885e1_f64) * t63304 + F::cast_from(0.22954444444444444444e0_f64) * t63306 - F::cast_from(0.38257407407407407407e0_f64) * t63308 - F::cast_from(0.68863333333333333334e0_f64) * t63313 - F::cast_from(0.34431666666666666667e0_f64) * t63317 + F::cast_from(0.91817777777777777776e0_f64) * t50826 - F::cast_from(0.34431666666666666666e0_f64) * t50828 - F::cast_from(0.10712074074074074074e1_f64) * t50834 - F::cast_from(0.61745185185185185186e-1_f64) * t63841 - F::cast_from(0.27785333333333333334e0_f64) * t63843 + F::cast_from(0.46308888888888888889e-1_f64) * t63845;
    t64132
}

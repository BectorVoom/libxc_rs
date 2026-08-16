//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2573/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2573<F: Float>(t50834: F, t51745: F, t63291: F, t63306: F, t63308: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F) -> F {
    let t72019 = -F::cast_from(0.34246666666666666666e-1_f64) * t63291 + F::cast_from(0.11415555555555555555e-1_f64) * t63306 - F::cast_from(0.19025925925925925925e-1_f64) * t63308 + t51745 - F::cast_from(0.5327259259259259259e-1_f64) * t50834 + F::cast_from(0.57077777777777777775e-1_f64) * t71124 - F::cast_from(0.1522074074074074074e-1_f64) * t63332 + F::cast_from(0.22831111111111111111e-1_f64) * t63334 - F::cast_from(0.17123333333333333333e-1_f64) * t63336 - F::cast_from(0.20547999999999999999e0_f64) * t71130 + F::cast_from(0.2283111111111111111e0_f64) * t71135 - F::cast_from(0.11415555555555555555e-1_f64) * t71140 + F::cast_from(0.11415555555555555555e-1_f64) * t71142 - F::cast_from(0.34246666666666666667e-1_f64) * t71144 - F::cast_from(0.63419753086419753083e-2_f64) * t71146 + F::cast_from(0.17123333333333333333e-1_f64) * t71150 - F::cast_from(0.34246666666666666667e-1_f64) * t71152 - F::cast_from(0.57077777777777777777e-2_f64) * t71154 + F::cast_from(0.2283111111111111111e-1_f64) * t71156 + F::cast_from(0.57077777777777777775e-1_f64) * t71160;
    t72019
}

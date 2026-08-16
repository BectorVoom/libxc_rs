//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2535/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2535<F: Float>(t50834: F, t51058: F, t63291: F, t63306: F, t63308: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F) -> F {
    let t71371 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t63291 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t63306 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t63308 + t51058 - F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t50834 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t71124 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t63332 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t63334 - t63336 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) * t71130 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t71135 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t71140 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t71142 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t71144 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t71146 + t71150 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t71152 - t71154 / F::cast_from(9.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t71156 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t71160;
    t71371
}

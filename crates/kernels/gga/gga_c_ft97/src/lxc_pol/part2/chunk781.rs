//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 781/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk781<F: Float>(t2253: F, t3655: F, t12143: F, t12144: F, t12148: F, t12152: F, t12155: F, t12158: F, t12162: F, t12164: F, t12165: F, t12171: F, t12174: F, t12177: F, t12181: F, t12186: F, t12190: F, t12193: F, t12198: F, t12201: F, t12204: F, t12236: F, t2265: F, t3628: F, t631: F) -> F {
    let t12240 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2253 * t3655;
    let t12241 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t12143 * t12144 - t2265 * t12148 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12143 * t12152 - t2265 * t12155 / F::cast_from(3.0_f64) - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12143 * t12158 + t12162 + t12164 + F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t12165 - F::cast_from(13.0_f64) / F::cast_from(9.0_f64) * t12171 + t12174 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2265 * t12177 - t2265 * t12181 / F::cast_from(3.0_f64) - t2265 * t12186 / F::cast_from(9.0_f64) - t12190 - F::cast_from(3.0_f64) * t631 * t12193 + F::cast_from(6.0_f64) * t631 * t12198 + t3628 * t12201 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t12204 + t631 * t12236 / F::cast_from(2.0_f64) - t12240;
    t12241
}

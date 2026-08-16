//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 930/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk930<F: Float>(t71238: F, t5214: F, t8232: F, t41955: F, t5209: F, t89: F, t5217: F, t9733: F, t5221: F, t5323: F, t5319: F, t5403: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t71239 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t71238;
    let t71276 = t8232 * t5214;
    let t71277 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t71276;
    let t71298 = t89 * t41955 * t5209;
    let t71299 = F::cast_from(8.0_f64) / F::cast_from(81.0_f64) * t71298;
    let t71305 = t89 * t9733 * t5217;
    let t71306 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t71305;
    let t71319 = t89 * t9733 * t5221;
    let t71320 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t71319;
    let t71363 = t8232 * t5323;
    let t71396 = t8232 * t5319;
    let t71522 = t8232 * t5403;
    (t71239, t71276, t71277, t71298, t71299, t71305, t71306, t71319, t71320, t71363, t71396, t71522)
}

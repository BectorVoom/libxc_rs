//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 825/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk825<F: Float>(t1771: F, t5356: F, t5352: F, t8282: F, t5346: F, t5349: F, t1636: F, t5226: F, t89: F, t5214: F, t8232: F, t41955: F, t5209: F, t5217: F, t9733: F, t5221: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t70801 = t1771 * t5356;
    let t70826 = t8282 * t5352;
    let t70935 = t8282 * t5346;
    let t70999 = t8282 * t5349;
    let t71238 = t89 * t1636 * t5226;
    let t71239 = 8.0 / 9.0 * t71238;
    let t71276 = t8232 * t5214;
    let t71277 = 8.0 / 27.0 * t71276;
    let t71298 = t89 * t41955 * t5209;
    let t71299 = 8.0 / 81.0 * t71298;
    let t71305 = t89 * t9733 * t5217;
    let t71306 = 8.0 / 27.0 * t71305;
    let t71319 = t89 * t9733 * t5221;
    (t70801, t70826, t70935, t70999, t71238, t71239, t71276, t71277, t71298, t71299, t71305, t71306, t71319)
}

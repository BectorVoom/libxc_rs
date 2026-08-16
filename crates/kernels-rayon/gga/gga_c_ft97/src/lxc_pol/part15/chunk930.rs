//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 930/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk930(t71238: f64, t5214: f64, t8232: f64, t41955: f64, t5209: f64, t89: f64, t5217: f64, t9733: f64, t5221: f64, t5323: f64, t5319: f64, t5403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t71239 = 8.0_f64 / 9.0_f64 * t71238;
    let t71276 = t8232 * t5214;
    let t71277 = 8.0_f64 / 27.0_f64 * t71276;
    let t71298 = t89 * t41955 * t5209;
    let t71299 = 8.0_f64 / 81.0_f64 * t71298;
    let t71305 = t89 * t9733 * t5217;
    let t71306 = 8.0_f64 / 27.0_f64 * t71305;
    let t71319 = t89 * t9733 * t5221;
    let t71320 = 4.0_f64 / 27.0_f64 * t71319;
    let t71363 = t8232 * t5323;
    let t71396 = t8232 * t5319;
    let t71522 = t8232 * t5403;
    (t71239, t71276, t71277, t71298, t71299, t71305, t71306, t71319, t71320, t71363, t71396, t71522)
}

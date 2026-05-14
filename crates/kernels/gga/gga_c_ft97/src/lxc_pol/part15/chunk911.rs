//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 911/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk911<F: Float>(t57435: F, t73256: F, t73259: F, t73262: F, t73276: F, t73299: F, t73301: F, t86016: F, t86020: F, t86172: F, t86175: F, t86178: F, t86181: F, t86188: F, t86195: F, t57491: F, t57527: F, t86199: F, t86202: F, t86205: F, t86208: F, t86211: F, t86214: F, t86217: F, t86220: F, t86223: F, t86226: F, t86232: F, t86236: F, t86240: F) -> (F, F) {
    let t86354 = -5.0 / 16.0 * t86016 - t86020 / 4.0 + t86172 / 6.0 - 4.0 * t86175 + 8.0 / 9.0 * t86178 + 8.0 / 3.0 * t86181 - 4.0 / 9.0 * t73256 + 8.0 / 9.0 * t73259 - 16.0 / 27.0 * t73262 + 4.0 / 27.0 * t73276 + 8.0 / 9.0 * t86188 + 16.0 / 27.0 * t57435 + 8.0 / 9.0 * t73299 + 8.0 / 9.0 * t73301 - 8.0 / 3.0 * t86195;
    let t86370 = -8.0 / 3.0 * t86199 + 4.0 / 9.0 * t86202 + 2.0 / 3.0 * t86205 + 8.0 / 3.0 * t86208 - 8.0 / 27.0 * t86211 + 4.0 / 9.0 * t86214 + 40.0 / 81.0 * t86217 - 20.0 / 27.0 * t86220 + 8.0 / 3.0 * t86223 + 8.0 / 3.0 * t86226 + 16.0 / 27.0 * t57491 - 16.0 / 81.0 * t57527 - 12.0 * t86232 - 8.0 / 3.0 * t86236 - 2.0 / 9.0 * t86240;
    (t86354, t86370)
}

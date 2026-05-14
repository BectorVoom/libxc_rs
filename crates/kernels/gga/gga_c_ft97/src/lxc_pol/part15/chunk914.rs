//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 914/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk914<F: Float>(t59339: F, t73256: F, t73259: F, t73262: F, t73276: F, t73299: F, t73301: F, t86175: F, t86178: F, t86181: F, t86188: F, t86195: F, t86199: F, t59354: F, t59364: F, t86202: F, t86205: F, t86208: F, t86211: F, t86214: F, t86217: F, t86220: F, t86223: F, t86226: F, t86232: F, t86236: F, t86240: F) -> (F, F) {
    let t86440 = -2.0 * t86175 + 4.0 / 9.0 * t86178 + 4.0 / 3.0 * t86181 - 2.0 / 9.0 * t73256 + 4.0 / 9.0 * t73259 - 8.0 / 27.0 * t73262 + 2.0 / 27.0 * t73276 + 4.0 / 9.0 * t86188 + t59339 + 4.0 / 9.0 * t73299 + 4.0 / 9.0 * t73301 - 4.0 / 3.0 * t86195 - 4.0 / 3.0 * t86199;
    let t86453 = 2.0 / 9.0 * t86202 + t86205 / 3.0 + 4.0 / 3.0 * t86208 - 4.0 / 27.0 * t86211 + 2.0 / 9.0 * t86214 + 20.0 / 81.0 * t86217 - 10.0 / 27.0 * t86220 + 4.0 / 3.0 * t86223 + 4.0 / 3.0 * t86226 + t59354 - t59364 - 6.0 * t86232 - 4.0 / 3.0 * t86236 - t86240 / 9.0;
    (t86440, t86453)
}

//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1216/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1216<F: Float>(t100252: F, t100214: F, t100219: F, t100224: F, t100228: F, t100232: F, t100236: F, t100241: F, t100246: F, t100250: F, t100257: F, t100262: F, t100270: F, t100272: F, t100277: F, t100283: F, t100288: F, t92192: F, t92195: F, t92201: F, t92219: F, t92238: F, t92240: F, t93728: F) -> (F, F) {
    let t102103 = 4.0 / 9.0 * t100252;
    let t102106 = 3.0 / 4.0 * t100214 - t100219 + 3.0 / 2.0 * t100224 - 2.0 / 3.0 * t100228 - 4.0 / 3.0 * t100232 + 2.0 / 9.0 * t100236 + t100241 / 2.0 + t100246 + 10.0 / 27.0 * t100250 + t102103 + t100257 / 4.0 + 2.0 * t100262;
    let t102109 = t100270 / 3.0;
    let t102110 = 2.0 * t100272;
    let t102113 = t93728 + t92192 + t92195 + 8.0 / 27.0 * t92201 + t92219 + t102109 + t102110 - t100277 / 2.0 - t92238 - t92240 - t100283 + t100288 / 3.0;
    (t102106, t102113)
}

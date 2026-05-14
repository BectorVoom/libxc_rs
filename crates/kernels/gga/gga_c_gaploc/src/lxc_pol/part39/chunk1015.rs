//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1015/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1015<F: Float>(t13818: F, t1599: F, t46953: F, t531: F, t557: F, t42230: F, t42233: F, t42236: F, t42239: F, t42242: F, t42245: F, t42250: F, t42254: F, t42257: F, t48093: F, t188: F, t189: F, t193: F, t46952: F) -> (F, F) {
    let t48096 = 0.35750489951850426669e0 * t1599 * t13818;
    let t48099 = 0.35750489951850426669e0 * t557 * t531 * t46953;
    let t48100 = -t48093 - t42230 + t42233 + t42236 + t42239 - t42242 + t42245 + t42250 + 0.42900587942220512003e1 * t42254 + t42257 - t48096 - t48099;
    let t48107 = 0.35750489951850426669e0 * t188 * t189 * t46952 * t193;
    (t48100, t48107)
}

//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 357/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk357<F: Float>(t3137: F, t492: F, t105: F, t3088: F, t3119: F, t3124: F, t3126: F, t3132: F, t3134: F, t921: F) -> (F, F, F) {
    let t3138 = t492 * t3137;
    let t3141 = 0.28455006635676149599e-1 * t105 * t3088 + 0.28455006635676149599e-1 * t105 * t3119 + t3124 - 0.85365019907028448797e-1 * t105 * t3126 - t3132 + 0.56910013271352299198e-1 * t105 * t3134 - 0.28455006635676149599e-1 * t105 * t3138;
    let t3145 = t921 * t921;
    (t3138, t3141, t3145)
}

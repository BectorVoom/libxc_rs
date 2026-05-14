//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1060/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1060<F: Float>(t13272: F, t32801: F, t122885: F, t45972: F, t125312: F, t2121: F, t2247: F, t136: F, t29411: F, t122886: F, t122890: F, t122901: F, t125265: F, t125274: F, t125319: F, t125324: F, t125332: F, t128403: F, t128411: F, t128415: F, t128424: F, t128428: F, t128434: F, t128451: F, t128457: F, t32593: F, t32602: F, t32795: F, t32798: F, t32802: F, t32806: F, t34177: F, t34181: F, t34410: F, t8737: F) -> (F,) {
    let t129213 = t13272 * t32801;
    let t129216 = t45972 * t122885;
    let t129232 = t2247 * t125312 * t2121;
    let t129236 = t2247 * t29411 * t136;
    let t129243 = 5.0 / 72.0 * t34410 * t32602 + 5.0 / 72.0 * t32795 * t34181 + 5.0 / 72.0 * t32806 * t34181 + 5.0 / 72.0 * t8737 * t128451 + 5.0 / 72.0 * t8737 * t125332 + 5.0 / 72.0 * t8737 * t128457 - 5.0 / 24.0 * t32798 * t125319 + 5.0 / 72.0 * t8737 * t125324 - 5.0 / 36.0 * t129213 * t32593 + 35.0 / 24.0 * t129216 * t128411 - 5.0 / 12.0 * t122886 * t128415 - 5.0 / 36.0 * t122901 * t34177 - 5.0 / 36.0 * t122890 * t34177 - 5.0 / 36.0 * t32802 * t125265 - 5.0 / 36.0 * t32802 * t128424 - 5.0 / 36.0 * t32802 * t128428 + 5.0 / 18.0 * t129232 * t128434 - 5.0 / 36.0 * t129236 * t32593 - 5.0 / 12.0 * t122886 * t128403 - 5.0 / 36.0 * t32802 * t125274;
    (t129243,)
}

//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1212/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1212(t13272: f64, t32801: f64, t122885: f64, t45972: f64, t125312: f64, t2121: f64, t2247: f64, t136: f64, t29411: f64, t122886: f64, t122890: f64, t122901: f64, t125265: f64, t125274: f64, t125319: f64, t125324: f64, t125332: f64, t128403: f64, t128411: f64, t128415: f64, t128424: f64, t128428: f64, t128434: f64, t128451: f64, t128457: f64, t32593: f64, t32602: f64, t32795: f64, t32798: f64, t32802: f64, t32806: f64, t34177: f64, t34181: f64, t34410: f64, t8737: f64) -> f64 {
    let t129213 = t13272 * t32801;
    let t129216 = t45972 * t122885;
    let t129232 = t2247 * t125312 * t2121;
    let t129236 = t2247 * t29411 * t136;
    let t129243 = 5.0_f64 / 72.0_f64 * t34410 * t32602 + 5.0_f64 / 72.0_f64 * t32795 * t34181 + 5.0_f64 / 72.0_f64 * t32806 * t34181 + 5.0_f64 / 72.0_f64 * t8737 * t128451 + 5.0_f64 / 72.0_f64 * t8737 * t125332 + 5.0_f64 / 72.0_f64 * t8737 * t128457 - 5.0_f64 / 24.0_f64 * t32798 * t125319 + 5.0_f64 / 72.0_f64 * t8737 * t125324 - 5.0_f64 / 36.0_f64 * t129213 * t32593 + 35.0_f64 / 24.0_f64 * t129216 * t128411 - 5.0_f64 / 12.0_f64 * t122886 * t128415 - 5.0_f64 / 36.0_f64 * t122901 * t34177 - 5.0_f64 / 36.0_f64 * t122890 * t34177 - 5.0_f64 / 36.0_f64 * t32802 * t125265 - 5.0_f64 / 36.0_f64 * t32802 * t128424 - 5.0_f64 / 36.0_f64 * t32802 * t128428 + 5.0_f64 / 18.0_f64 * t129232 * t128434 - 5.0_f64 / 36.0_f64 * t129236 * t32593 - 5.0_f64 / 12.0_f64 * t122886 * t128403 - 5.0_f64 / 36.0_f64 * t32802 * t125274;
    t129243
}

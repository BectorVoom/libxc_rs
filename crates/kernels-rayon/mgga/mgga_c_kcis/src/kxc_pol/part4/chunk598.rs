//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 598/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk598(t1122: f64, t3178: f64, t1092: f64, t1130: f64, t982: f64, t1133: f64, t1021: f64, t89: f64, t828: f64, t2635: f64, t8: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3179 = t3178 * t1122;
    let t3180 = t1092 * t3179;
    let t3182 = t982 * t1130;
    let t3183 = t3182 * t1133;
    let t3184 = t1021 * t3183;
    let t3185 = t1092 * t3184;
    let t3187 = 2.0_f64 * t89;
    let t3188 = 2.0_f64 * t828;
    let t3190 = t2635 * t8 + t3187 - t3188;
    (t3179, t3180, t3182, t3183, t3184, t3185, t3187, t3188, t3190)
}

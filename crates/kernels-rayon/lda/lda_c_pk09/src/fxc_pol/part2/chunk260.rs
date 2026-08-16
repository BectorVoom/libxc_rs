//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 260/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk260(t1161: f64, t272: f64, t1156: f64, t42: f64, t560: f64, t561: f64, t256: f64, t263: f64, t1: f64, t262: f64, t261: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1162 = t272 * t1161;
    let t1164 = 1.28_f64 * t1156 * t1162;
    let t1165 = 1.1801314654631911_f64 * t42;
    let t1166 = 4.033992295431624_f64 * t560;
    let t1167 = 4.594893021405177_f64 * t561;
    let t1168 = t256 * t263;
    let t1169 = 1.7849970861284015_f64 * t1168;
    let t1171 = 1.0_f64 / t262 / t1;
    let t1173 = 0.0439648946916576_f64 * t261 * t1171;
    let t1174 = t1165 - t1166 + t1167 - t1169 + t1173;
    let t1175 = t1174 * t271;
    (t1162, t1164, t1165, t1166, t1167, t1168, t1169, t1171, t1173, t1174, t1175)
}

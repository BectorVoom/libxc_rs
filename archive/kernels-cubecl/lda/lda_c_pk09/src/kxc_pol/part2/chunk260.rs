//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 260/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk260<F: Float>(t1161: F, t272: F, t1156: F, t42: F, t560: F, t561: F, t256: F, t263: F, t1: F, t262: F, t261: F, t271: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1162 = t272 * t1161;
    let t1164 = F::cast_from(1.28_f64) * t1156 * t1162;
    let t1165 = F::cast_from(1.1801314654631911_f64) * t42;
    let t1166 = F::cast_from(4.033992295431624_f64) * t560;
    let t1167 = F::cast_from(4.594893021405177_f64) * t561;
    let t1168 = t256 * t263;
    let t1169 = F::cast_from(1.7849970861284015_f64) * t1168;
    let t1171 = F::cast_from(1.0_f64) / t262 / t1;
    let t1173 = F::cast_from(0.0439648946916576_f64) * t261 * t1171;
    let t1174 = t1165 - t1166 + t1167 - t1169 + t1173;
    let t1175 = t1174 * t271;
    (t1162, t1164, t1165, t1166, t1167, t1168, t1169, t1171, t1173, t1174, t1175)
}

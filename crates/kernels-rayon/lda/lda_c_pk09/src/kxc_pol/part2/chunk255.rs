//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 255/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk255(t44: f64, t1127: f64, t776: f64, t879: f64, t938: f64, t7: f64, t620: f64, t413: f64, t13: f64, t236: f64, t229: f64, t243: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t1129 = t776 + t879 + t938 + t1127;
    let t1130 = t7 * t1129;
    let t1134 = piecewise3(t45, 0.0_f64, 2.0_f64 * t44 * t620);
    let t1135 = t1134 * t413;
    let t1137 = t13 * t236;
    let t1139 = t229 * t243;
    (t1129, t1130, t1134, t1135, t1137, t1139)
}

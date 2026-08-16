//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 267/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk267(t44: f64, t1196: f64, t1197: f64, t1189: f64, t1193: f64, t1195: f64, t276: f64, t1192: f64, t51: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t1198 = t1196 * t1197;
    let t1201 = t1189 * t1193 + 1.28_f64 * t1195 * t1198;
    let t1202 = t276 * t1201;
    let t1203 = piecewise3(t45, t1192, t1202);
    let t1204 = f64::ln(t51);
    (t1202, t1203, t1204)
}

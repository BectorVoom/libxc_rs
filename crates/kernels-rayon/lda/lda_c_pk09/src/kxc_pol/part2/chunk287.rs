//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 287/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk287(t1244: f64, t1251: f64, t1256: f64, t1259: f64, t1264: f64, t1268: f64, t1273: f64, t1275: f64, t353: f64, t306: f64, t300: f64, t337: f64) -> (f64, f64, f64, f64, f64) {
    let t1277 = t1244 - 3.2084841915276807_f64 * t1251 + t1256 + 3.2084841915276807_f64 * t1259 + t1264 - 0.64_f64 * t1268 + t1273 + 0.64_f64 * t1275;
    let t1278 = 1.0_f64 / t353;
    let t1279 = t1277 * t1278;
    let t1280 = t1279 * t306;
    let t1283 = t300 * t337;
    (t1277, t1278, t1279, t1280, t1283)
}

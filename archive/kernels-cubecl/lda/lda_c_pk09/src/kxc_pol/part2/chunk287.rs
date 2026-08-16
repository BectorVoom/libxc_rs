//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 287/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk287<F: Float>(t1244: F, t1251: F, t1256: F, t1259: F, t1264: F, t1268: F, t1273: F, t1275: F, t353: F, t306: F, t300: F, t337: F) -> (F, F, F, F, F) {
    let t1277 = t1244 - F::cast_from(3.2084841915276807_f64) * t1251 + t1256 + F::cast_from(3.2084841915276807_f64) * t1259 + t1264 - F::cast_from(0.64_f64) * t1268 + t1273 + F::cast_from(0.64_f64) * t1275;
    let t1278 = F::cast_from(1.0_f64) / t353;
    let t1279 = t1277 * t1278;
    let t1280 = t1279 * t306;
    let t1283 = t300 * t337;
    (t1277, t1278, t1279, t1280, t1283)
}

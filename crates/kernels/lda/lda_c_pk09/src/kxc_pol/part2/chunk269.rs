//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 269/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk269<F: Float>(t51: F, t1206: F, t1207: F, t1189: F, t1195: F, t1204: F, t278: F, t1192: F, t1203: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t1208 = t1206 * t1207;
    let t1211 = t1189 * t1204 + F::new(1.28) * t1195 * t1208;
    let t1212 = t278 * t1211;
    let t1213 = piecewise3::<f64>(t52, t1192, t1212);
    let t1214 = t1203 + t1213;
    (t1212, t1214)
}

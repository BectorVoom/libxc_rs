//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 981/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk981<F: Float>(t2257: F, t26: F, t329: F, t1322: F, t5882: F, t1316: F, t1317: F, t1324: F, t2308: F, t346: F, t4234: F, t4355: F, t5883: F, t8057: F, t8070: F, t8074: F, t8077: F, t8081: F, t8087: F, t8091: F, t8092: F, t8094: F, t8110: F, t8115: F, t8474: F) -> F {
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11645 = t5882 * t1322;
    let t11664 = -F::new(18.0) * t11640 * t4234 - F::new(6.0) * t8474 * t4355 - F::new(3.0) * t346 * t11645 * t1324 - F::new(3.0) * t346 * t2308 * t8057 - F::new(3.0) * t346 * t2308 * t8110 - t346 * t2308 * t8115 + F::new(9.0) * t1316 * t5883 * t1317 - F::cast_from(0.0001639671923854359_f64) * t8070 - t8074 + F::cast_from(0.0004919015771563077_f64) * t8077 + t8081 - t8087 - t8091 - F::cast_from(0.15965645347006147_f64) * t8092 - F::cast_from(0.47896936041018434_f64) * t8094;
    t11664
}

//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 981/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk981(t2257: f64, t26: f64, t329: f64, t1322: f64, t5882: f64, t1316: f64, t1317: f64, t1324: f64, t2308: f64, t346: f64, t4234: f64, t4355: f64, t5883: f64, t8057: f64, t8070: f64, t8074: f64, t8077: f64, t8081: f64, t8087: f64, t8091: f64, t8092: f64, t8094: f64, t8110: f64, t8115: f64, t8474: f64) -> f64 {
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11645 = t5882 * t1322;
    let t11664 = -18.0_f64 * t11640 * t4234 - 6.0_f64 * t8474 * t4355 - 3.0_f64 * t346 * t11645 * t1324 - 3.0_f64 * t346 * t2308 * t8057 - 3.0_f64 * t346 * t2308 * t8110 - t346 * t2308 * t8115 + 9.0_f64 * t1316 * t5883 * t1317 - 0.0001639671923854359_f64 * t8070 - t8074 + 0.0004919015771563077_f64 * t8077 + t8081 - t8087 - t8091 - 0.15965645347006147_f64 * t8092 - 0.47896936041018434_f64 * t8094;
    t11664
}

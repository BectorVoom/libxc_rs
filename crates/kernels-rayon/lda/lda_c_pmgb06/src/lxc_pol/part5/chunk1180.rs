//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1180/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1180(t73: f64, t7306: f64, t11200: f64, t1316: f64, t19124: f64, t19140: f64, t19148: f64, t21267: f64, t2236: f64, t2308: f64, t2311: f64, t2718: f64, t342: f64, t346: f64, t374: f64, t384: f64, t388: f64, t4232: f64, t5583: f64, t5903: f64, t6006: f64, t6007: f64, t6018: f64, t61: f64, t7041: f64, t7086: f64, t7089: f64, t769: f64, t783: f64, t787: f64, t7881: f64, t790: f64, t7909: f64, t7920: f64, t8070: f64, t8074: f64, t8077: f64) -> f64 {
    let t21278 = t73 * t7306;
    let t21305 = -9.0_f64 * t5583 * t4232 * t2236 * t769 - 3.0_f64 * t5583 * t4232 * t2718 * t342 - t346 * t2308 * t73 * t7041 + 18.0_f64 * t11200 * t7909 + (t19124 + t19140 + t19148 + t21267) * t61 + 9.0_f64 * t1316 * t7089 * t2311 + 2.0_f64 * t346 * t5903 * t384 * t7881 + 3.0_f64 * t1316 * t388 * t21278 + 2.0_f64 * t346 * t7089 * t787 + t346 * t790 * t7086 + 4.0_f64 * t6006 * t6007 * t783 * t2236 - 2.0_f64 * t346 * t2308 * t787 * t2236 + 18.0_f64 * t5583 * t6007 * t7920 * t374 - 18.0_f64 * t6018 * t4232 * t7920 * t342 - 5.4655730795145296e-05_f64 * t8070 - t8074 + 0.0001639671923854359_f64 * t8077;
    t21305
}

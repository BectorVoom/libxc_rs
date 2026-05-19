//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1180/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1180<F: Float>(t73: F, t7306: F, t11200: F, t1316: F, t19124: F, t19140: F, t19148: F, t21267: F, t2236: F, t2308: F, t2311: F, t2718: F, t342: F, t346: F, t374: F, t384: F, t388: F, t4232: F, t5583: F, t5903: F, t6006: F, t6007: F, t6018: F, t61: F, t7041: F, t7086: F, t7089: F, t769: F, t783: F, t787: F, t7881: F, t790: F, t7909: F, t7920: F, t8070: F, t8074: F, t8077: F) -> F {
    let t21278 = t73 * t7306;
    let t21305 = -F::new(9.0) * t5583 * t4232 * t2236 * t769 - F::new(3.0) * t5583 * t4232 * t2718 * t342 - t346 * t2308 * t73 * t7041 + F::new(18.0) * t11200 * t7909 + (t19124 + t19140 + t19148 + t21267) * t61 + F::new(9.0) * t1316 * t7089 * t2311 + F::new(2.0) * t346 * t5903 * t384 * t7881 + F::new(3.0) * t1316 * t388 * t21278 + F::new(2.0) * t346 * t7089 * t787 + t346 * t790 * t7086 + F::new(4.0) * t6006 * t6007 * t783 * t2236 - F::new(2.0) * t346 * t2308 * t787 * t2236 + F::new(18.0) * t5583 * t6007 * t7920 * t374 - F::new(18.0) * t6018 * t4232 * t7920 * t342 - F::cast_from(5.4655730795145296e-05_f64) * t8070 - t8074 + F::cast_from(0.0001639671923854359_f64) * t8077;
    t21305
}

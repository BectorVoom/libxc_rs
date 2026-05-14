//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 993/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk993<F: Float>(t3476: F, t521: F, t1458: F, t3518: F, t1245: F, t537: F, t188: F, t1: F, t1184: F, t2071: F, t548: F, t3604: F, t5165: F, t219: F, t4048: F, t3589: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11867 = t521 * t3476;
    let t11871 = t1458 * t3518;
    let t11875 = t537 * t1245;
    let t11879 = t188 * t1245;
    let t11898 = t1 * t1184;
    let t11900 = t548 * t11898 * t2071;
    let t11907 = t5165 * t3604;
    let t11913 = t4048 * t219;
    let t11914 = t11913 * t3589;
    (t11867, t11871, t11875, t11879, t11898, t11900, t11907, t11913, t11914)
}

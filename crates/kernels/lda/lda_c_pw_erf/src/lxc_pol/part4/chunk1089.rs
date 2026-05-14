//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1089/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1089<F: Float>(t6330: F, t945: F, t11: F, t503: F, t2329: F, t3476: F, t940: F, t1243: F, t1245: F, t5992: F, t348: F, t6335: F, t3518: F, t3536: F, t6351: F, t2430: F, t925: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15858 = t6330 * t945;
    let t15860 = t11 * t503 * t15858;
    let t15863 = t3476 * t2329 * t940;
    let t15865 = t11 * t1243 * t15863;
    let t15867 = t1245 * t5992;
    let t15868 = t15867 * t348;
    let t15870 = t11 * t1243 * t15868;
    let t15872 = t6335 * t945;
    let t15874 = t11 * t1243 * t15872;
    let t15877 = t3518 * t2329 * t940;
    let t15879 = t11 * t3536 * t15877;
    let t15881 = t6351 * t940;
    let t15883 = t11 * t503 * t15881;
    let t15887 = t925 * t2430;
    (t15858, t15860, t15863, t15865, t15868, t15870, t15872, t15874, t15877, t15879, t15881, t15883, t15887)
}

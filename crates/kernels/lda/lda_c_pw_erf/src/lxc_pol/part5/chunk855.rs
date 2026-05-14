//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 855/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk855<F: Float>(t2114: F, t4564: F, t1529: F, t1960: F, t1466: F, t3667: F, t1401: F, t3899: F, t3476: F, t5146: F, t197: F, t3892: F, t3518: F, t2120: F, t3550: F, t3553: F, t795: F) -> (F, F, F, F, F, F, F, F) {
    let t11946 = t2114 * t4564;
    let t11947 = 8.0 / 45.0 * t11946;
    let t11954 = t1960 * t1529;
    let t11955 = 4.0 / 45.0 * t11954;
    let t11983 = t1466 * t3667;
    let t11989 = t3899 * t1401;
    let t12025 = t5146 * t3476;
    let t12030 = t3892 * t197;
    let t12031 = t12030 * t3518;
    let t12046 = t2120 * t3550;
    let t12047 = 8.0 / 45.0 * t12046;
    let t12050 = t795 * t3553;
    (t11947, t11955, t11983, t11989, t12025, t12031, t12047, t12050)
}

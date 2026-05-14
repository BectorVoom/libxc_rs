//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1017/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1017<F: Float>(t1333: F, t4507: F, t10030: F, t5157: F, t5162: F, t2140: F, t3742: F, t2143: F, t3745: F, t1476: F, t5334: F, t1124: F, t188: F, t1325: F, t4958: F, t2171: F, t3735: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13122 = t4507 * t1333;
    let t13128 = t10030 * t5157;
    let t13130 = t10030 * t5162;
    let t13144 = t3742 * t2140;
    let t13146 = t3745 * t2143;
    let t13163 = t5334 * t1476;
    let t13172 = t1124 * t188;
    let t13174 = t1325 * t13172 * t4958;
    let t13176 = t2171 * t3735;
    (t13122, t13128, t13130, t13144, t13146, t13163, t13172, t13174, t13176)
}

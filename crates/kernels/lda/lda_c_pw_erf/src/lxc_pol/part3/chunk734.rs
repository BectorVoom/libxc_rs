//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 734/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk734<F: Float>(t1522: F, t820: F, t184: F, t1333: F, t811: F, t951: F, t1319: F, t2006: F, t3859: F, t1325: F, t1251: F, t784: F, t940: F, t1326: F, t1976: F, t348: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5400 = t1522 * t820;
    let t5401 = t5400 * t184;
    let t5404 = t811 * t1333;
    let t5405 = t5404 * t951;
    let t5406 = t1319 * t5405;
    let t5409 = t3859 * t2006;
    let t5411 = 32.0 / 135.0 * t1325 * t5409;
    let t5412 = t784 * t1251;
    let t5413 = t5412 * t940;
    let t5414 = t1326 * t5413;
    let t5417 = t1976 * t348;
    (t5400, t5401, t5405, t5406, t5409, t5411, t5413, t5414, t5417)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 642/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk642<F: Float>(t1124: F, t521: F, t2177: F, t519: F, t1472: F, t2140: F, t2157: F, t3899: F, t1318: F, t2182: F, t3787: F, t1325: F) -> (F, F, F, F, F, F, F, F) {
    let t4906 = t1124 * t521;
    let t4907 = t4906 * t2177;
    let t4908 = t519 * t4907;
    let t4917 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t1472 * t2140;
    let t4933 = t3899 * t2157;
    let t4935 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1318 * t4933;
    let t4946 = t3787 * t2182;
    let t4948 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1325 * t4946;
    (t4906, t4907, t4908, t4917, t4933, t4935, t4946, t4948)
}

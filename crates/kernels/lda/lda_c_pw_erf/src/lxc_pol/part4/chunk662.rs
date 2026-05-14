//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 662/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk662<F: Float>(t22: F, t4048: F, t1475: F, t1479: F, t571: F, t1484: F, t9: F, t1487: F, t1496: F, t202: F, t184: F, t1210: F, t168: F, t671: F, t1534: F, t635: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4049 = t22 * t4048;
    let t4059 = t1475 * t1479;
    let t4060 = t571 * t4059;
    let t4062 = t9 * t1484;
    let t4063 = t4062 * t1487;
    let t4064 = t571 * t4063;
    let t4072 = t202 * t1496;
    let t4073 = t4072 * t184;
    let t4084 = t168 * t1210 * t671;
    let t4087 = t168 * t635 * t1534;
    (t4049, t4059, t4060, t4062, t4063, t4064, t4072, t4073, t4084, t4087)
}

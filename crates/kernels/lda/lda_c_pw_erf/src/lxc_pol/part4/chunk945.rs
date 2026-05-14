//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 945/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk945<F: Float>(t185: F, t9248: F, t1498: F, t1529: F, t1612: F, t1621: F, t4062: F, t581: F, t1390: F, t1449: F, t3762: F, t1309: F, t571: F, t1508: F, t1519: F, t1351: F, t212: F, t22: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9250 = 112.0 / 1215.0 * t185 * t9248;
    let t9251 = t1498 * t1529;
    let t9253 = t1612 * t1621;
    let t9278 = t4062 * t581;
    let t9304 = t1449 * t1390;
    let t9313 = t3762 * t581;
    let t9315 = t571 * t9313 * t1309;
    let t9380 = t1508 * t1519;
    let t9408 = t22 / t212 / t1351;
    (t9250, t9251, t9253, t9278, t9304, t9313, t9315, t9380, t9408)
}

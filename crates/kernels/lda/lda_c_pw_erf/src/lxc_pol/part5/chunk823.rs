//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 823/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk823<F: Float>(t9409: F, t155: F, t213: F, t2151: F, t576: F, t571: F, t2070: F, t548: F, t550: F, t1401: F, t1475: F, t3893: F, t529: F, t3883: F, t1251: F, t177: F, t191: F) -> (F, F, F, F, F, F, F, F) {
    let t9410 = 1.0 / t9409;
    let t9432 = t155 * t213;
    let t9436 = t2151 * t576;
    let t9437 = t571 * t9436;
    let t9593 = t548 * t2070 * t550;
    let t9678 = t1475 * t1401;
    let t9700 = t3893 * t529;
    let t9723 = t3883 * t529;
    let t9761 = t191 / t177 / t1251;
    (t9410, t9432, t9437, t9593, t9678, t9700, t9723, t9761)
}

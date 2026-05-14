//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 948/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk948<F: Float>(t1401: F, t1475: F, t3883: F, t529: F, t4072: F, t518: F, t1251: F, t177: F, t191: F, t1244: F, t2061: F, t539: F, t1250: F, t1275: F, t933: F, t1269: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9678 = t1475 * t1401;
    let t9723 = t3883 * t529;
    let t9752 = t4072 * t518;
    let t9761 = t191 / t177 / t1251;
    let t9762 = t1244 * t1244;
    let t9763 = 1.0 / t9762;
    let t9772 = t2061 * t539;
    let t9777 = 1.0 / t1244 / t1250;
    let t9782 = t933 * t1275;
    let t9784 = t933 * t1269;
    (t9678, t9723, t9752, t9761, t9763, t9772, t9777, t9782, t9784)
}

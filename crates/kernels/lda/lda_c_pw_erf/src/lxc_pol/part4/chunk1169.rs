//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1169/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1169<F: Float>(t331: F, t6558: F, t1268: F, t15783: F, t15790: F, t15798: F, t15800: F, t15804: F, t15809: F, t15813: F, t15818: F, t16377: F, t17226: F, t1971: F, t2061: F, t538: F, t9813: F) -> (F,) {
    let t17234 = t331 * t6558;
    let t17236 = 0.026660493827160493 * t15798 + 0.3519185185185185 * t15800 + 0.14396666666666666 * t15804 + 0.47988888888888886 * t15809 - 0.03999074074074074 * t15813 - 0.10664197530864197 * t15818 + 0.05333333333333334 * t2061 * t538 * t1971 - 0.007407407407407408 * t17226 - 0.017777777777777778 * t16377 * t1268 * t15790 + 0.10666666666666667 * t16377 * t538 * t15783 + 0.008888888888888889 * t17234 + t9813;
    (t17236,)
}

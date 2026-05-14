//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1170/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1170<F: Float>(t5021: F, t6528: F, t11793: F, t11798: F, t11803: F, t11805: F, t1268: F, t15820: F, t15827: F, t15840: F, t15844: F, t15858: F, t15863: F, t15868: F, t15881: F, t25: F, t538: F) -> (F,) {
    let t17249 = t5021 * t6528;
    let t17263 = 0.14396666666666666 * t15820 - 0.017777777777777778 * t11793 + 0.05333333333333334 * t11798 + 0.002962962962962963 * t11803 + 0.003950617283950617 * t11805 - 0.31992592592592595 * t15827 + 0.16 * t25 * t538 * t15881 - 0.08 * t25 * t1268 * t15844 - 0.2311111111111111 * t17249 - 0.04 * t25 * t538 * t15840 + 0.013333333333333334 * t25 * t538 * t15858 + 0.013333333333333334 * t25 * t1268 * t15863 - 0.0044444444444444444 * t25 * t1268 * t15868;
    (t17263,)
}

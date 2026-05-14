//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 712/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk712<F: Float>(t156: F, t2619: F, t426: F, t2624: F, t431: F, t325: F, t2627: F, t128: F, t6121: F, t10: F, t2599: F, t415: F, t2611: F, t3313: F, t3322: F, t5598: F, t5609: F, t7143: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7145 = t156 * t2619;
    let t7146 = t426 * t7145;
    let t7148 = t431 * t2624;
    let t7149 = t7148 * t325;
    let t7151 = t431 * t2627;
    let t7152 = t7151 * t325;
    let t7154 = t128 * t6121;
    let t7155 = t10 * t7154;
    let t7158 = t415 * t2599;
    let t7159 = t7158 * t325;
    let t7160 = 0.9743416666666667 * t7159;
    let t7161 = t415 * t2611;
    let t7162 = t7161 * t325;
    let t7163 = 0.48717083333333333 * t7162;
    let t7164 = -t5598 - t5609 - t7143 / 2.0 + t7146 / 6.0 - 2.93808 * t7149 + 0.73452 * t7152 - t426 * t7155 / 2.0 - t7160 + t7163 + t3313 - t3322;
    (t7145, t7146, t7148, t7149, t7151, t7152, t7154, t7155, t7158, t7159, t7160, t7161, t7162, t7163, t7164)
}

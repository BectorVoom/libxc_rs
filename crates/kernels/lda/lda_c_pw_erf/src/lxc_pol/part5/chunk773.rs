//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 773/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk773<F: Float>(t2627: F, t411: F, t156: F, t2615: F, t426: F, t2619: F, t2624: F, t431: F, t325: F, t128: F, t6121: F, t10: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7137 = t2627 * t411;
    let t7142 = t156 * t2615;
    let t7143 = t426 * t7142;
    let t7145 = t156 * t2619;
    let t7146 = t426 * t7145;
    let t7148 = t431 * t2624;
    let t7149 = t7148 * t325;
    let t7151 = t431 * t2627;
    let t7152 = t7151 * t325;
    let t7154 = t128 * t6121;
    let t7155 = t10 * t7154;
    (t7137, t7142, t7143, t7145, t7146, t7148, t7149, t7151, t7152, t7154, t7155)
}

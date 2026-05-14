//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 856/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk856<F: Float>(t1597: F, t2877: F, t2916: F, t2826: F, t485: F, t1112: F, t1124: F, t483: F, t1131: F, t4166: F, t1191: F, t465: F, t1138: F, t1578: F, t2910: F, t4259: F) -> (F, F, F, F, F, F, F, F) {
    let t10800 = 0.013871971944573394 * t2877 * t2916 * t1597;
    let t10802 = 0.12408369628826103 * t2826 * t485;
    let t10805 = t1124 * t1112 * t483 * t485;
    let t10808 = t4166 * t1131 * t485;
    let t10810 = t1191 * t465;
    let t10812 = t10810 * t1138 * t1597;
    let t10816 = 0.03950778065781896 * t1578 * t2910 * t485;
    let t10817 = 0.7561297733553868 * t4259;
    (t10800, t10802, t10805, t10808, t10810, t10812, t10816, t10817)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 951/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk951<F: Float>(t1597: F, t2819: F, t2853: F, t473: F, t483: F, t485: F, t2877: F, t2916: F, t2826: F, t1112: F, t1124: F, t1131: F, t4166: F) -> (F, F, F, F, F, F) {
    let t10793 = F::cast_from(0.02267957317922317_f64) * t2819 * t1597;
    let t10796 = t473 * t2853 * t483 * t485;
    let t10800 = F::cast_from(0.013871971944573394_f64) * t2877 * t2916 * t1597;
    let t10802 = F::cast_from(0.12408369628826103_f64) * t2826 * t485;
    let t10805 = t1124 * t1112 * t483 * t485;
    let t10808 = t4166 * t1131 * t485;
    (t10793, t10796, t10800, t10802, t10805, t10808)
}

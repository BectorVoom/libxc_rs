//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 521/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk521<F: Float>(t159: F, t285: F, t2853: F, t1112: F, t477: F, t281: F, t1128: F, t465: F, t1184: F, t6: F) -> (F, F, F, F, F, F) {
    let t2855 = t2853 * t159 * t285;
    let t2859 = t1112 * t477 * t285;
    let t2860 = t281 * t2859;
    let t2863 = t465 * t1128 * t285;
    let t2864 = t281 * t2863;
    let t2869 = t6 * t1184;
    (t2855, t2859, t2860, t2863, t2864, t2869)
}

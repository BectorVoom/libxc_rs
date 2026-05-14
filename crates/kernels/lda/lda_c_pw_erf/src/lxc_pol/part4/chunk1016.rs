//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1016/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1016<F: Float>(t13080: F, t1318: F, t4759: F, t4804: F, t5409: F, t3794: F, t1325: F, t3859: F, t5275: F, t5237: F, t5265: F, t3704: F, t3973: F) -> (F, F, F, F, F, F) {
    let t13082 = t1318 * t13080 * t4759;
    let t13097 = t4804 * t5409;
    let t13099 = t3794 * t5409;
    let t13102 = t1325 * t3859 * t5275;
    let t13105 = t1325 * t5237 * t5265;
    let t13115 = t3973 * t3704;
    (t13082, t13097, t13099, t13102, t13105, t13115)
}

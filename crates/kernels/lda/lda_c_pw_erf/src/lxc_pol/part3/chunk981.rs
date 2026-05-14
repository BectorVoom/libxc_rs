//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 981/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk981<F: Float>(t4804: F, t5409: F, t3794: F, t1325: F, t3859: F, t5275: F, t5237: F, t5265: F, t5155: F, t954: F, t3974: F, t5166: F, t951: F, t11914: F, t3704: F, t3973: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13097 = t4804 * t5409;
    let t13098 = 32.0 / 45.0 * t13097;
    let t13099 = t3794 * t5409;
    let t13100 = 32.0 / 45.0 * t13099;
    let t13102 = t1325 * t3859 * t5275;
    let t13103 = 16.0 / 45.0 * t13102;
    let t13105 = t1325 * t5237 * t5265;
    let t13106 = 16.0 / 27.0 * t13105;
    let t13107 = t5155 * t954;
    let t13110 = 8.0 / 9.0 * t3974 * t5166 * t13107;
    let t13111 = t5155 * t951;
    let t13114 = 64.0 / 27.0 * t3974 * t11914 * t13111;
    let t13115 = t3973 * t3704;
    (t13098, t13100, t13103, t13106, t13107, t13110, t13111, t13114, t13115)
}

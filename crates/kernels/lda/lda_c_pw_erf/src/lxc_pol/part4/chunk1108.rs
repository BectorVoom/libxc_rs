//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1108/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1108<F: Float>(t16159: F, t4804: F, t6433: F, t3794: F, t1325: F, t2098: F, t5289: F, t542: F, t784: F, t2171: F, t4856: F, t4859: F, t4862: F, t1992: F, t5327: F, t5247: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16160 = 64.0 / 45.0 * t16159;
    let t16162 = 32.0 / 15.0 * t4804 * t6433;
    let t16164 = 32.0 / 15.0 * t3794 * t6433;
    let t16169 = 32.0 / 15.0 * t1325 * t5289 * t784 * t2098 * t542;
    let t16171 = 16.0 / 45.0 * t2171 * t4856;
    let t16173 = 16.0 / 9.0 * t2171 * t4859;
    let t16175 = 64.0 / 45.0 * t2171 * t4862;
    let t16177 = 16.0 / 27.0 * t5327 * t1992;
    let t16179 = 8.0 / 27.0 * t2171 * t5247;
    (t16160, t16162, t16164, t16169, t16171, t16173, t16175, t16177, t16179)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1235/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1235<F: Float>(t16306: F, t2017: F, t571: F, t1325: F, t1991: F, t2429: F, t944: F, t1472: F, t6397: F, t6401: F, t12794: F, t2385: F, t12797: F, t10656: F, t4753: F, t6479: F) -> (F, F, F, F, F, F, F, F) {
    let t18346 = 8.0 / 27.0 * t571 * t2017 * t16306;
    let t18350 = 8.0 / 27.0 * t1325 * t1991 * t2429 * t944;
    let t18352 = 16.0 / 45.0 * t1472 * t6397;
    let t18354 = 32.0 / 45.0 * t1472 * t6401;
    let t18356 = 16.0 / 45.0 * t12794 * t2385;
    let t18358 = 32.0 / 45.0 * t12797 * t2385;
    let t18359 = 32.0 / 405.0 * t10656;
    let t18361 = 32.0 / 45.0 * t4753 * t6479;
    (t18346, t18350, t18352, t18354, t18356, t18358, t18359, t18361)
}

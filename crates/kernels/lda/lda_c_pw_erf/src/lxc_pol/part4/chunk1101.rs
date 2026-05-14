//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1101/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1101<F: Float>(t2171: F, t5296: F, t519: F, t6427: F, t9304: F, t1251: F, t2471: F, t3806: F, t940: F, t13432: F, t6464: F, t1325: F, t3859: F, t6468: F, t2389: F, t3742: F) -> (F, F, F, F, F, F) {
    let t16040 = 16.0 / 45.0 * t2171 * t5296;
    let t16042 = t519 * t9304 * t6427;
    let t16043 = 32.0 / 135.0 * t16042;
    let t16048 = 16.0 / 45.0 * t519 * t3806 * t2471 * t1251 * t940;
    let t16050 = t519 * t13432 * t6464;
    let t16051 = 16.0 / 27.0 * t16050;
    let t16053 = t1325 * t3859 * t6468;
    let t16054 = 64.0 / 135.0 * t16053;
    let t16056 = 16.0 / 45.0 * t3742 * t2389;
    (t16040, t16043, t16048, t16051, t16054, t16056)
}

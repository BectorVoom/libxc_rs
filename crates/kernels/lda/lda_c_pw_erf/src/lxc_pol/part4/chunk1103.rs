//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1103/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1103<F: Float>(t16075: F, t1472: F, t6375: F, t1381: F, t2411: F, t3832: F, t571: F, t3802: F, t519: F, t6488: F, t1392: F, t2433: F, t3806: F, t16040: F, t16043: F, t16048: F, t16051: F, t16054: F, t16056: F, t16059: F, t16063: F, t16066: F, t16067: F, t16070: F, t16073: F) -> (F, F, F, F, F, F) {
    let t16076 = 80.0 / 81.0 * t16075;
    let t16078 = 8.0 / 27.0 * t1472 * t6375;
    let t16082 = 4.0 / 27.0 * t571 * t3832 * t2411 * t1381;
    let t16084 = t519 * t3802 * t6488;
    let t16085 = 32.0 / 135.0 * t16084;
    let t16089 = 16.0 / 45.0 * t519 * t3806 * t2433 * t1392;
    let t16090 = t16040 + t16043 - t16048 + t16051 - t16054 - t16056 + t16059 + t16063 - t16066 + t16067 - t16070 + t16073 + t16076 - t16078 - t16082 + t16085 - t16089;
    (t16076, t16078, t16082, t16085, t16089, t16090)
}

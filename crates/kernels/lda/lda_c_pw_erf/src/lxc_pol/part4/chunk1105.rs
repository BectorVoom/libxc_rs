//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1105/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1105<F: Float>(t1287: F, t1318: F, t2017: F, t2411: F, t15685: F, t5418: F, t4738: F, t5292: F, t2023: F, t5334: F, t558: F, t6843: F, t1308: F, t352: F, t571: F, t13080: F, t6446: F) -> (F, F, F, F, F, F) {
    let t16114 = 8.0 / 27.0 * t1318 * t2017 * t2411 * t1287;
    let t16116 = 32.0 / 45.0 * t15685 * t5418;
    let t16118 = 32.0 / 15.0 * t4738 * t5292;
    let t16120 = 16.0 / 45.0 * t5334 * t2023;
    let t16121 = t6843 * t558;
    let t16125 = 8.0 / 45.0 * t571 * t1308 * t16121 * t352;
    let t16127 = t571 * t13080 * t6446;
    (t16114, t16116, t16118, t16120, t16125, t16127)
}

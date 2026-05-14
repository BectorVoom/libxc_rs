//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 839/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk839<F: Float>(t325: F, t3643: F, t3651: F, t1353: F, t925: F, t3634: F, t4048: F, t56: F, t1361: F, t9812: F, t155: F, t188: F, t1325: F, t1442: F, t2176: F, t524: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10094 = t325 * t3643;
    let t10096 = t325 * t3651;
    let t10098 = t925 * t1353;
    let t10100 = t325 * t3634;
    let t10102 = t56 * t4048;
    let t10115 = t925 * t1361;
    let t10145 = 0.01959135802469136 * t9812;
    let t10162 = t155 * t188;
    let t10164 = t1325 * t10162 * t1442;
    let t10166 = t2176 * t524;
    (t10094, t10096, t10098, t10100, t10102, t10115, t10145, t10162, t10164, t10166)
}

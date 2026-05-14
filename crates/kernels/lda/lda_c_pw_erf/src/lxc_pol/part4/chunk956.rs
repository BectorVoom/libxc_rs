//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 956/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk956<F: Float>(t10162: F, t1325: F, t1442: F, t2176: F, t524: F, t519: F, t1612: F, t610: F, t1155: F, t603: F, t10042: F, t2061: F, t590: F, t1375: F, t933: F, t1378: F) -> (F, F, F, F, F, F, F, F) {
    let t10164 = t1325 * t10162 * t1442;
    let t10166 = t2176 * t524;
    let t10167 = t519 * t10166;
    let t10169 = t1612 * t610;
    let t10172 = 0.004413481481481482 * t1155 * t603;
    let t10195 = 0.3732469135802469 * t10042;
    let t10202 = t2061 * t590;
    let t10204 = t933 * t1375;
    let t10206 = t933 * t1378;
    (t10164, t10167, t10169, t10172, t10195, t10202, t10204, t10206)
}

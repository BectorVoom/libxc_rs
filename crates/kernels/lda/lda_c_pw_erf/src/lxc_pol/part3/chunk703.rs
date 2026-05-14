//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 703/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk703<F: Float>(t4930: F, t571: F, t2157: F, t3899: F, t1318: F, t1392: F, t3675: F, t806: F, t1440: F, t519: F, t1401: F, t1403: F, t811: F, t1466: F, t2182: F, t3787: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4932 = 4.0 / 5.0 * t571 * t4930;
    let t4933 = t3899 * t2157;
    let t4935 = 16.0 / 45.0 * t1318 * t4933;
    let t4937 = t3675 * t806 * t1392;
    let t4938 = t1440 * t4937;
    let t4940 = 4.0 / 5.0 * t519 * t4938;
    let t4942 = t1401 * t811 * t1403;
    let t4943 = t1466 * t4942;
    let t4945 = 8.0 / 15.0 * t1318 * t4943;
    let t4946 = t3787 * t2182;
    (t4932, t4933, t4935, t4937, t4938, t4940, t4942, t4943, t4945, t4946)
}

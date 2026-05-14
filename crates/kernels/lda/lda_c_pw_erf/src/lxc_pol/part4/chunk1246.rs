//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1246/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1246<F: Float>(t12916: F, t1318: F, t6957: F, t1484: F, t1486: F, t2824: F, t35: F, t571: F, t2143: F, t5327: F, t2171: F, t4907: F, t4753: F, t6894: F, t3416: F, t1466: F, t2191: F, t5029: F) -> (F, F, F, F, F, F, F) {
    let t18510 = t1318 * t12916 * t6957;
    let t18511 = 8.0 / 9.0 * t18510;
    let t18516 = 32.0 / 27.0 * t571 * t2824 * t1484 * t1486 * t35;
    let t18517 = t5327 * t2143;
    let t18518 = 32.0 / 135.0 * t18517;
    let t18519 = t2171 * t4907;
    let t18520 = 16.0 / 27.0 * t18519;
    let t18521 = t4753 * t6894;
    let t18522 = 32.0 / 45.0 * t18521;
    let t18523 = t3416 * t6894;
    let t18524 = 32.0 / 45.0 * t18523;
    let t18528 = 8.0 / 15.0 * t1318 * t1466 * t2191 * t5029;
    (t18511, t18516, t18518, t18520, t18522, t18524, t18528)
}

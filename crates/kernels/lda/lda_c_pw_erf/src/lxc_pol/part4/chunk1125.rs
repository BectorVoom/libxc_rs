//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1125/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1125<F: Float>(t4763: F, t4933: F, t1403: F, t1466: F, t2526: F, t3667: F, t571: F, t16246: F, t16251: F, t16254: F, t16259: F, t16262: F, t16264: F, t16266: F, t16268: F, t16269: F, t16270: F, t16271: F, t16513: F, t16515: F, t16517: F, t16519: F) -> (F, F, F) {
    let t16520 = t4763 * t4933;
    let t16521 = 32.0 / 45.0 * t16520;
    let t16526 = 4.0 / 5.0 * t571 * t1466 * t3667 * t2526 * t1403;
    let t16527 = t16246 - t16251 + t16254 - t16259 - t16262 - t16264 - t16266 - t16268 + t16269 - t16270 + t16271 - t16513 + t16515 + t16517 - t16519 - t16521 - t16526;
    (t16521, t16526, t16527)
}

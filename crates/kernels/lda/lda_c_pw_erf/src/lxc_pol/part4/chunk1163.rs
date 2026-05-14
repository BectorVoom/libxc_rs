//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1163/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1163<F: Float>(t17114: F, t1518: F, t185: F, t2498: F, t12031: F, t16004: F, t3965: F, t1318: F, t3899: F, t6992: F, t1381: F, t1466: F, t6991: F, t1336: F, t6205: F, t2146: F, t4930: F) -> (F, F, F, F, F, F, F) {
    let t17115 = 32.0 / 45.0 * t17114;
    let t17117 = t185 * t1518 * t2498;
    let t17118 = 4.0 / 135.0 * t17117;
    let t17121 = 128.0 / 81.0 * t3965 * t12031 * t16004;
    let t17123 = t1318 * t3899 * t6992;
    let t17124 = 16.0 / 45.0 * t17123;
    let t17128 = 4.0 / 15.0 * t1318 * t1466 * t6991 * t1381;
    let t17130 = 8.0 / 45.0 * t6205 * t1336;
    let t17132 = 8.0 / 5.0 * t2146 * t4930;
    (t17115, t17118, t17121, t17124, t17128, t17130, t17132)
}

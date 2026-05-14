//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1105/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1105<F: Float>(t1447: F, t6509: F, t5499: F, t6513: F, t332: F, t477: F, t6637: F, t13308: F, t5077: F, t12677: F, t493: F, t5318: F, t6119: F, t486: F, t6610: F, t5115: F, t802: F) -> (F, F, F, F, F, F, F, F) {
    let t16522 = t1447 * t6509;
    let t16523 = 32.0 / 243.0 * t16522;
    let t16524 = t5499 * t6513;
    let t16525 = 20.0 / 81.0 * t16524;
    let t16527 = t6637 * t477 * t332;
    let t16530 = 16.0 / 45.0 * t5077 * t13308 * t16527;
    let t16531 = 4.0 / 15.0 * t12677;
    let t16534 = 2.0 / 15.0 * t493 * t6119 * t5318;
    let t16535 = t486 * t6610;
    let t16536 = 4.0 / 45.0 * t16535;
    let t16537 = t802 * t5115;
    (t16523, t16525, t16527, t16530, t16531, t16534, t16536, t16537)
}

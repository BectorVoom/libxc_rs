//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1127/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1127<F: Float>(t16545: F, t4488: F, t4494: F, t4501: F, t12314: F, t5152: F, t2337: F, t593: F, t3974: F, t3976: F, t549: F, t2065: F, t743: F, t10027: F, t6743: F, t6749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16548 = 16.0 / 45.0 * t4488 * t4494 * t16545;
    let t16551 = 8.0 / 27.0 * t4488 * t4501 * t16545;
    let t16553 = 32.0 / 45.0 * t12314 * t5152;
    let t16554 = t2337 * t593;
    let t16558 = 16.0 / 45.0 * t3974 * t3976 * t16554 * t549;
    let t16559 = t743 * t2065;
    let t16563 = 32.0 / 45.0 * t3974 * t3976 * t16559 * t549;
    let t16565 = 32.0 / 45.0 * t10027 * t6743;
    let t16567 = 64.0 / 45.0 * t10027 * t6749;
    (t16548, t16551, t16553, t16554, t16558, t16559, t16563, t16565, t16567)
}

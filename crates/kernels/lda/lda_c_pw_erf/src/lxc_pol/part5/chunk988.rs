//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 988/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk988<F: Float>(t4506: F, t4508: F, t6396: F, t17645: F, t1949: F, t12428: F, t1944: F, t2466: F, t10011: F, t7749: F, t16867: F, t2030: F, t4488: F, t4490: F, t6460: F, t16863: F, t2026: F, t3965: F) -> (F, F, F, F, F, F, F) {
    let t20861 = 16.0 / 15.0 * t4506 * t4508 * t6396;
    let t20864 = 16.0 / 15.0 * t4506 * t17645 * t1949;
    let t20868 = 8.0 / 9.0 * t4506 * t12428 * t2466 * t1944;
    let t20869 = t10011 * t7749;
    let t20870 = 32.0 / 45.0 * t20869;
    let t20873 = 8.0 / 5.0 * t4488 * t16867 * t2030;
    let t20876 = 16.0 / 15.0 * t4488 * t4490 * t6460;
    let t20879 = 16.0 / 15.0 * t3965 * t16863 * t2026;
    (t20861, t20864, t20868, t20870, t20873, t20876, t20879)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 983/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk983<F: Float>(t20777: F, t4488: F, t542: F, t6710: F, t2329: F, t784: F, t3965: F, t3967: F, t348: F, t5141: F, t5147: F, t2337: F, t743: F, t593: F, t4506: F, t4515: F) -> (F, F, F, F, F, F, F) {
    let t20795 = 8.0 / 15.0 * t4488 * t6710 * t20777 * t542;
    let t20796 = t2329 * t784;
    let t20800 = 8.0 / 15.0 * t3965 * t3967 * t20796 * t542;
    let t20801 = t20796 * t348;
    let t20804 = 16.0 / 15.0 * t3965 * t5141 * t20801;
    let t20807 = 8.0 / 9.0 * t3965 * t5147 * t20801;
    let t20808 = t2337 * t743;
    let t20809 = t20808 * t593;
    let t20812 = 8.0 / 15.0 * t4506 * t4515 * t20809;
    (t20795, t20800, t20804, t20807, t20808, t20809, t20812)
}

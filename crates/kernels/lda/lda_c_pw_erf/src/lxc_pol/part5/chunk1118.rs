//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1118/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1118<F: Float>(t20729: F, t3965: F, t5141: F, t20777: F, t3967: F, t494: F, t4488: F, t542: F, t6710: F, t2329: F, t784: F, t348: F) -> (F, F, F, F, F) {
    let t20787 = F::new(16.0) / F::new(15.0) * t3965 * t5141 * t20729;
    let t20791 = F::new(8.0) / F::new(15.0) * t3965 * t3967 * t20777 * t494;
    let t20795 = F::new(8.0) / F::new(15.0) * t4488 * t6710 * t20777 * t542;
    let t20796 = t2329 * t784;
    let t20800 = F::new(8.0) / F::new(15.0) * t3965 * t3967 * t20796 * t542;
    let t20801 = t20796 * t348;
    (t20787, t20791, t20795, t20800, t20801)
}

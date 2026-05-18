//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1119/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1119<F: Float>(t20801: F, t3965: F, t5141: F, t5147: F, t2337: F, t743: F, t593: F, t4506: F, t4515: F, t352: F) -> (F, F, F, F, F, F) {
    let t20804 = F::new(16.0) / F::new(15.0) * t3965 * t5141 * t20801;
    let t20807 = F::new(8.0) / F::new(9.0) * t3965 * t5147 * t20801;
    let t20808 = t2337 * t743;
    let t20809 = t20808 * t593;
    let t20812 = F::new(8.0) / F::new(15.0) * t4506 * t4515 * t20809;
    let t20813 = t20808 * t352;
    (t20804, t20807, t20808, t20809, t20812, t20813)
}

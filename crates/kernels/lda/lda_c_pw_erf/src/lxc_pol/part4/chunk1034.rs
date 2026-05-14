//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1034/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1034<F: Float>(t1639: F, t20: F, t5794: F, t1926: F, t4196: F, t4199: F, t4546: F, t4207: F, t4589: F, t515: F, t172: F, t184: F, t4645: F, t3859: F, t4637: F, t519: F) -> (F, F, F, F, F, F, F) {
    let t14098 = t5794 * t20 * t1639;
    let t14100 = t1926 * t4196;
    let t14103 = t4546 * t4199;
    let t14105 = t4546 * t4207;
    let t14107 = t4589 * t515;
    let t14110 = t172 * t4645 * t184;
    let t14190 = t519 * t3859 * t4637;
    (t14098, t14100, t14103, t14105, t14107, t14110, t14190)
}

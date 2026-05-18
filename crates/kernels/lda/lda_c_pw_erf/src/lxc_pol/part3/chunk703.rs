//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 703/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk703<F: Float>(t4410: F, t1077: F, t1765: F, t1: F, t1750: F, t887: F, t1755: F, t1746: F, t1769: F, t2951: F, t1904: F, t462: F) -> (F, F, F, F, F, F, F, F) {
    let t4411 = F::new(0.019751789702565206) * t4410;
    let t4412 = t1765 * t1077;
    let t4413 = F::new(1.169644679491041) * t4412;
    let t4415 = t887 * t1750 * t1;
    let t4416 = t4415 * t1755;
    let t4418 = t1769 * t1746;
    let t4420 = F::new(2.339289358982082) * t2951;
    let t4422 = t462 * t1904;
    (t4411, t4412, t4413, t4415, t4416, t4418, t4420, t4422)
}

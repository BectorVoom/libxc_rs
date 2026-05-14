//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1002/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1002<F: Float>(t3476: F, t4500: F, t10011: F, t5138: F, t5143: F, t5148: F, t3704: F, t3964: F) -> (F, F, F, F, F) {
    let t12439 = t4500 * t3476;
    let t12460 = t10011 * t5138;
    let t12462 = t10011 * t5143;
    let t12464 = t10011 * t5148;
    let t12475 = t3964 * t3704;
    (t12439, t12460, t12462, t12464, t12475)
}

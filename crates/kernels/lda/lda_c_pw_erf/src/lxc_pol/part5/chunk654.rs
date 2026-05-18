//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 654/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk654<F: Float>(t1325: F, t5378: F, t2187: F, t3787: F, t519: F, t1333: F, t811: F, t2006: F, t3859: F, t1251: F, t784: F, t1996: F, t3802: F) -> (F, F, F, F, F, F, F, F) {
    let t5380 = F::new(16.0) / F::new(45.0) * t1325 * t5378;
    let t5397 = t3787 * t2187;
    let t5399 = F::new(16.0) / F::new(45.0) * t519 * t5397;
    let t5404 = t811 * t1333;
    let t5409 = t3859 * t2006;
    let t5411 = F::new(32.0) / F::new(135.0) * t1325 * t5409;
    let t5412 = t784 * t1251;
    let t5421 = t3802 * t1996;
    (t5380, t5397, t5399, t5404, t5409, t5411, t5412, t5421)
}

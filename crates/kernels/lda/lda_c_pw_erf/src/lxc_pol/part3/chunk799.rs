//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 799/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk799<F: Float>(t2181: F, t944: F, t1440: F, t2187: F, t3787: F, t519: F, t1522: F, t820: F, t184: F, t1333: F, t811: F, t951: F) -> (F, F, F, F, F, F, F) {
    let t5393 = t2181 * t944;
    let t5394 = t1440 * t5393;
    let t5397 = t3787 * t2187;
    let t5399 = F::new(16.0) / F::new(45.0) * t519 * t5397;
    let t5400 = t1522 * t820;
    let t5401 = t5400 * t184;
    let t5404 = t811 * t1333;
    let t5405 = t5404 * t951;
    (t5393, t5394, t5397, t5399, t5400, t5401, t5405)
}

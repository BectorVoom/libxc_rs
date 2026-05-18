//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 905/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk905<F: Float>(t211: F, t3670: F, t514: F, t218: F, t3666: F, t1513: F, t1519: F, t3437: F, t565: F, t198: F, t4567: F, t185: F) -> (F, F, F, F, F, F) {
    let t9234 = t211 * t514 * t3670;
    let t9237 = F::new(1.0) / t3666 / t218;
    let t9244 = t1513 * t1519;
    let t9246 = t565 * t3437;
    let t9248 = t4567 * t198;
    let t9250 = F::new(112.0) / F::new(1215.0) * t185 * t9248;
    (t9234, t9237, t9244, t9246, t9248, t9250)
}

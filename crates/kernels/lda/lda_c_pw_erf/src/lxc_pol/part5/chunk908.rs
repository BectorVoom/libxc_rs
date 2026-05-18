//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 908/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk908<F: Float>(t196: F, t3674: F, t218: F, t3666: F, t3437: F, t565: F, t198: F, t4567: F, t185: F, t4062: F, t581: F, t3667: F, t574: F) -> (F, F, F, F, F, F) {
    let t9223 = F::new(1.0) / t3674 / t196;
    let t9237 = F::new(1.0) / t3666 / t218;
    let t9246 = t565 * t3437;
    let t9248 = t4567 * t198;
    let t9250 = F::new(112.0) / F::new(1215.0) * t185 * t9248;
    let t9278 = t4062 * t581;
    let t9286 = t574 * t3667;
    (t9223, t9237, t9246, t9250, t9278, t9286)
}

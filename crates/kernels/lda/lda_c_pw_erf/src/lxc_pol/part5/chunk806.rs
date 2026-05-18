//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 806/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk806<F: Float>(t7376: F, t85: F, t2995: F, t3000: F, t3009: F, t3011: F, t3016: F, t3118: F, t3121: F, t3125: F, t3133: F, t3155: F, t7353: F) -> (F, F) {
    let t7377 = t7376 * t85;
    let t7378 = F::new(0.019751789702565206) * t7377;
    let t7379 = -t7353 + t2995 - t3000 - t3009 - t3011 + t3016 + t7378 + t3155 + t3118 - t3121 + t3125 + t3133;
    (t7378, t7379)
}

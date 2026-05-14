//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 816/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk816<F: Float>(t4459: F, t5727: F, t5743: F, t5942: F, t312: F, t19: F, t2316: F, t729: F, t734: F, t2343: F, t75: F, t402: F, t2705: F, t4387: F, t4389: F, t4391: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5944 = t4459 + t5727 + t5743 + t5942;
    let t5945 = t5944 * t312;
    let t5949 = t2316 * t729 * t19;
    let t5950 = t5949 * t734;
    let t5967 = t2343 * t75;
    let t5968 = t5967 * t402;
    let t5969 = 0.5848223397455204 * t5968;
    let t5970 = 0.010843580882781523 * t2705;
    let t5971 = 0.0004883081343134176 * t4387;
    let t5972 = 1.169644679491041 * t4389;
    let t5973 = 34.631511798751724 * t4391;
    (t5944, t5945, t5949, t5950, t5967, t5968, t5969, t5970, t5971, t5972, t5973)
}

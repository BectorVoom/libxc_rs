//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 334/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk334<F: Float>(t191: F, t299: F, t187: F, t190: F, t331: F, t539: F, t176: F, t177: F) -> (F, F, F, F, F) {
    let t1260 = t299 * t191;
    let t1263 = F::new(0.011111111111111112) * t190 * t1260 * t187;
    let t1264 = t331 * t539;
    let t1267 = F::new(1.0) / t177 / t176;
    let t1268 = t191 * t1267;
    (t1260, t1263, t1264, t1267, t1268)
}

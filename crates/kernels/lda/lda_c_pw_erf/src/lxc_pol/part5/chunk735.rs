//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 735/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk735<F: Float>(t6125: F, t7060: F, t7079: F, t7305: F, t312: F, t19: F, t2686: F, t729: F, t734: F, t5968: F, t4387: F, t4389: F, t4391: F, t4398: F, t4401: F, t4403: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7307 = t6125 + t7060 + t7079 + t7305;
    let t7308 = t7307 * t312;
    let t7314 = t2686 * t729 * t19;
    let t7315 = t7314 * t734;
    let t7323 = 1.7544670192365612 * t5968;
    let t7324 = 0.0007324622014701264 * t4387;
    let t7325 = 1.7544670192365612 * t4389;
    let t7326 = 51.94726769812759 * t4391;
    let t7327 = 0.032530742648344574 * t4398;
    let t7328 = 36.0 * t4401;
    let t7329 = 96.0 * t4403;
    (t7307, t7308, t7314, t7315, t7323, t7324, t7325, t7326, t7327, t7328, t7329)
}

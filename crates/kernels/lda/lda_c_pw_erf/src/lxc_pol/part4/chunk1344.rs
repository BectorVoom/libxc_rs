//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1344/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1344<F: Float>(t168: F, t635: F, t7025: F, t1125: F, t153: F, t2357: F, t632: F, t7045: F, t15483: F, t242: F, t11196: F, t11198: F, t11204: F, t11211: F, t14932: F, t14935: F, t14938: F) -> (F,) {
    let t19358 = t168 * t635 * t7025;
    let t19361 = t153 * t1125 * t2357;
    let t19363 = t7045 * t632;
    let t19365 = t15483 * t242;
    let t19372 = -t11196 + 0.3891025816905257 * t11198 + 0.039794582218349216 * t19358 + 1.328721022894618 * t19361 + 0.1675256410710088 * t19363 + 0.1675256410710088 * t19365 - 8.858140152630787 * t11204 - 0.053059442957798957 * t11211 + 5.314884091578472 * t14932 - 8.858140152630787 * t14935 + 0.039794582218349216 * t14938;
    (t19372,)
}

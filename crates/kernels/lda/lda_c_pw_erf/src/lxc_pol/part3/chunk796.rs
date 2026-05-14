//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 796/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk796<F: Float>(t2946: F, t386: F, t400: F, t8171: F, t1059: F, t2942: F, t1009: F, t1026: F, t2986: F, t1027: F, t1030: F, t8428: F, t1073: F, t3007: F, t1184: F, t119: F, t395: F, t84: F) -> (F, F, F, F, F, F, F) {
    let t8437 = 14.03573615389249 * t400 * t2946 * t8171 * t386;
    let t8438 = t1059 * t2942;
    let t8441 = 1.0 / t1026 / t1009;
    let t8445 = 12304.676425209354 * t400 * t8441 * t8171 * t2986;
    let t8449 = 51.94726769812759 * t400 * t1027 * t8428 * t1030;
    let t8464 = t1073 * t3007;
    let t8469 = 0.0018989760778855128 * t395 * t119 * t1184 * t84;
    (t8437, t8438, t8441, t8445, t8449, t8464, t8469)
}

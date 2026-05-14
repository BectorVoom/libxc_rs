//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 898/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk898<F: Float>(t331: F, t6802: F, t2420: F, t925: F, t2412: F, t325: F, t6651: F, t4606: F, t6654: F, t6824: F, t6827: F, t5021: F, t6830: F, t6818: F, t6821: F, t2140: F, t5334: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16382 = t331 * t6802;
    let t16397 = t925 * t2420;
    let t16399 = t925 * t2412;
    let t16432 = t325 * t6651;
    let t16434 = t4606 * t6654;
    let t16439 = t331 * t6824;
    let t16441 = t331 * t6827;
    let t16445 = t5021 * t6830;
    let t16468 = t331 * t6818;
    let t16470 = t331 * t6821;
    let t16514 = t5334 * t2140;
    (t16382, t16397, t16399, t16432, t16434, t16439, t16441, t16445, t16468, t16470, t16514)
}

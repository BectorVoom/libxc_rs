//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 928/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk928<F: Float>(t325: F, t3624: F, t1953: F, t560: F, t1357: F, t925: F, t3643: F, t3651: F, t1353: F, t3634: F, t4048: F, t56: F) -> (F, F, F, F, F, F, F, F) {
    let t10079 = t325 * t3624;
    let t10090 = t1953 * t560;
    let t10092 = t925 * t1357;
    let t10094 = t325 * t3643;
    let t10096 = t325 * t3651;
    let t10098 = t925 * t1353;
    let t10100 = t325 * t3634;
    let t10102 = t56 * t4048;
    (t10079, t10090, t10092, t10094, t10096, t10098, t10100, t10102)
}

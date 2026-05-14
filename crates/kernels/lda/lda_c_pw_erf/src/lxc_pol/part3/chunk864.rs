//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 864/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk864<F: Float>(t1155: F, t646: F, t4100: F, t10682: F, t3921: F, t256: F, t3939: F, t652: F, t19: F, t2853: F, t644: F, t647: F, t1415: F, t1432: F, t1427: F, t3946: F) -> (F, F, F, F, F, F, F) {
    let t11029 = 0.19208479012345678 * t1155 * t646;
    let t11035 = t4100 * t646;
    let t11038 = 0.008082336938271605 * t10682 * t3921;
    let t11046 = t3939 * t652 * t256;
    let t11050 = t2853 * t19 * t644 * t647;
    let t11053 = t1415 * t1432 * t256;
    let t11055 = t3946 * t1427;
    (t11029, t11035, t11038, t11046, t11050, t11053, t11055)
}

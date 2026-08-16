//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1072/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1072<F: Float>(t5381: F, t588: F, t97: F, t4159: F, t871: F, t1560: F, t5220: F, t443: F, t464: F, t1423: F, t5291: F, t1992: F, t3457: F) -> (F, F, F, F, F, F) {
    let t11930 = t5381 * t97 * t588;
    let t11944 = t871 * t4159;
    let t11952 = t5220 * t1560;
    let t11966 = t464 * t443;
    let t11971 = t1423 * t5291;
    let t12006 = t1992 * t3457;
    (t11930, t11944, t11952, t11966, t11971, t12006)
}

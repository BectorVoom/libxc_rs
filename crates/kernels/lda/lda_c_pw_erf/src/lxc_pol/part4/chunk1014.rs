//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1014/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1014<F: Float>(t10654: F, t1949: F, t571: F, t3863: F, t4837: F, t519: F, t5221: F, t9723: F, t4581: F, t4753: F, t3416: F, t1318: F, t3854: F, t5225: F, t4794: F, t5230: F) -> (F, F, F, F, F, F, F) {
    let t13051 = t571 * t10654 * t1949;
    let t13054 = t571 * t3863 * t4837;
    let t13066 = t519 * t9723 * t5221;
    let t13068 = t4753 * t4581;
    let t13070 = t3416 * t4581;
    let t13073 = t1318 * t3854 * t5225;
    let t13078 = t1318 * t4794 * t5230;
    (t13051, t13054, t13066, t13068, t13070, t13073, t13078)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1028/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1028<F: Float>(t10011: F, t6771: F, t10030: F, t6756: F, t2337: F, t352: F, t3863: F, t571: F, t6396: F, t13444: F, t6400: F, t13080: F, t1318: F, t6482: F) -> (F, F, F, F, F, F) {
    let t17657 = t10011 * t6771;
    let t17664 = t10030 * t6756;
    let t17673 = t2337 * t352;
    let t17684 = t571 * t3863 * t6396;
    let t17687 = t571 * t13444 * t6400;
    let t17690 = t1318 * t13080 * t6482;
    (t17657, t17664, t17673, t17684, t17687, t17690)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1232/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1232<F: Float>(t14049: F, t14052: F, t184: F, t1958: F, t549: F, t813: F, t3794: F, t6469: F, t1325: F, t1326: F, t2433: F, t944: F, t519: F, t5237: F, t6352: F, t3863: F, t571: F, t6356: F) -> (F, F, F, F, F, F, F) {
    let t18295 = 32.0 / 405.0 * t14049;
    let t18296 = 64.0 / 405.0 * t14052;
    let t18300 = 16.0 / 15.0 * t549 * t1958 * t184 * t813;
    let t18302 = 32.0 / 45.0 * t3794 * t6469;
    let t18306 = 16.0 / 45.0 * t1325 * t1326 * t2433 * t944;
    let t18308 = t519 * t5237 * t6352;
    let t18309 = 32.0 / 27.0 * t18308;
    let t18311 = t571 * t3863 * t6356;
    (t18295, t18296, t18300, t18302, t18306, t18309, t18311)
}

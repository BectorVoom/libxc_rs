//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1031/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1031<F: Float>(t1926: F, t4204: F, t4183: F, t185: F, t4567: F, t4723: F, t1298: F, t4564: F, t4568: F, t2162: F, t571: F, t9432: F, t3899: F, t5374: F, t10030: F, t5167: F) -> (F, F, F, F, F, F, F, F) {
    let t13917 = t1926 * t4204;
    let t13919 = t1926 * t4183;
    let t13922 = t185 * t4567 * t4723;
    let t13924 = t1298 * t4564;
    let t13926 = t1298 * t4568;
    let t13929 = t571 * t9432 * t2162;
    let t13932 = t571 * t3899 * t5374;
    let t13938 = t10030 * t5167;
    (t13917, t13919, t13922, t13924, t13926, t13929, t13932, t13938)
}

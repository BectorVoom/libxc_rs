//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1022/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1022<F: Float>(t1318: F, t3854: F, t5405: F, t1472: F, t4788: F, t4791: F, t4795: F, t4906: F, t529: F, t4849: F, t519: F, t12695: F, t4633: F, t1124: F, t1458: F, t197: F) -> (F, F, F, F, F, F, F, F) {
    let t13422 = t1318 * t3854 * t5405;
    let t13426 = t1472 * t4788;
    let t13428 = t1472 * t4791;
    let t13430 = t1472 * t4795;
    let t13432 = t4906 * t529;
    let t13434 = t519 * t13432 * t4849;
    let t13437 = t519 * t12695 * t4633;
    let t13440 = t1124 * t1458 * t197;
    (t13422, t13426, t13428, t13430, t13432, t13434, t13437, t13440)
}

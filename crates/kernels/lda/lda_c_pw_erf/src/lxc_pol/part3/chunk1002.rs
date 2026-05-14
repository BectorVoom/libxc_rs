//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1002/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1002<F: Float>(t10654: F, t1318: F, t2001: F, t3854: F, t5405: F, t2171: F, t3808: F, t1472: F, t4788: F, t4791: F, t4795: F, t4906: F, t529: F, t4849: F, t519: F, t12695: F, t4633: F) -> (F, F, F, F, F, F, F, F) {
    let t13419 = t1318 * t10654 * t2001;
    let t13420 = 16.0 / 135.0 * t13419;
    let t13422 = t1318 * t3854 * t5405;
    let t13423 = 32.0 / 45.0 * t13422;
    let t13425 = 8.0 / 15.0 * t2171 * t3808;
    let t13426 = t1472 * t4788;
    let t13427 = 16.0 / 45.0 * t13426;
    let t13428 = t1472 * t4791;
    let t13429 = 32.0 / 45.0 * t13428;
    let t13430 = t1472 * t4795;
    let t13431 = 16.0 / 27.0 * t13430;
    let t13432 = t4906 * t529;
    let t13434 = t519 * t13432 * t4849;
    let t13435 = 8.0 / 9.0 * t13434;
    let t13437 = t519 * t12695 * t4633;
    (t13420, t13423, t13425, t13427, t13429, t13431, t13435, t13437)
}

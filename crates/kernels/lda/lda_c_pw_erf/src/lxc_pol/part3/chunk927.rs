//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 927/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk927<F: Float>(t1351: F, t2065: F, t3832: F, t571: F, t951: F, t10379: F, t2967: F, t3589: F, t833: F, t2171: F, t3847: F, t3404: F, t2035: F, t9752: F, t3851: F, t2002: F) -> (F, F, F, F, F, F, F) {
    let t12234 = 4.0 / 9.0 * t571 * t3832 * t2065 * t1351 * t951;
    let t12239 = 32.0 / 81.0 * t571 * t10379 * t833 * t3589 * t2967;
    let t12241 = 4.0 / 15.0 * t2171 * t3847;
    let t12243 = 4.0 / 9.0 * t2171 * t3404;
    let t12245 = 8.0 / 15.0 * t9752 * t2035;
    let t12247 = 4.0 / 15.0 * t2171 * t3851;
    let t12249 = 8.0 / 15.0 * t9752 * t2002;
    (t12234, t12239, t12241, t12243, t12245, t12247, t12249)
}

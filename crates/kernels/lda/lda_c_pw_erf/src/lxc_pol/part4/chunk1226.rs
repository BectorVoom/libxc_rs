//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1226/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1226<F: Float>(t18192: F, t13938: F, t13975: F, t13977: F, t10015: F, t6771: F, t12329: F, t6713: F, t6717: F, t6720: F, t16847: F, t3965: F, t3967: F, t494: F, t10465: F, t10469: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18193 = 64.0 / 135.0 * t18192;
    let t18194 = 64.0 / 81.0 * t13938;
    let t18195 = 32.0 / 45.0 * t13975;
    let t18196 = 32.0 / 45.0 * t13977;
    let t18198 = 32.0 / 45.0 * t10015 * t6771;
    let t18200 = 32.0 / 45.0 * t12329 * t6713;
    let t18202 = 32.0 / 45.0 * t12329 * t6717;
    let t18204 = 16.0 / 27.0 * t12329 * t6720;
    let t18208 = 32.0 / 45.0 * t3965 * t3967 * t16847 * t494;
    let t18209 = 32.0 / 405.0 * t10465;
    let t18210 = 16.0 / 405.0 * t10469;
    (t18193, t18194, t18195, t18196, t18198, t18200, t18202, t18204, t18208, t18209, t18210)
}

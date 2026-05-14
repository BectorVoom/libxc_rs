//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 842/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk842<F: Float>(t226: F, t4606: F, t5021: F, t7: F, t1397: F, t4073: F, t1472: F, t3748: F, t1453: F, t3783: F, t519: F, t1458: F, t155: F, t1461: F, t3723: F, t3883: F) -> (F, F, F, F, F, F, F) {
    let t10286 = 4.0 / 3.0 * t226 * (-4.277777777777778 * t4606 + 220.0 / 81.0 * t5021) * M_PI * t7;
    let t10294 = t4073 * t1397;
    let t10296 = t1472 * t3748;
    let t10311 = t519 * t3783 * t1453;
    let t10313 = t155 * t1458;
    let t10315 = t519 * t10313 * t1461;
    let t10318 = t519 * t3883 * t3723;
    (t10286, t10294, t10296, t10311, t10313, t10315, t10318)
}

//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1135/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1135<F: Float>(t2171: F, t5222: F, t3867: F, t571: F, t6270: F, t954: F, t10397: F, t1351: F, t2466: F, t951: F, t1318: F, t1319: F, t6275: F, t4738: F, t5414: F, t5418: F) -> (F, F, F, F, F, F) {
    let t16683 = 8.0 / 27.0 * t2171 * t5222;
    let t16687 = 8.0 / 45.0 * t571 * t3867 * t6270 * t954;
    let t16692 = 8.0 / 27.0 * t571 * t10397 * t2466 * t1351 * t951;
    let t16696 = 8.0 / 45.0 * t1318 * t1319 * t6275 * t954;
    let t16698 = 32.0 / 45.0 * t4738 * t5414;
    let t16700 = 32.0 / 45.0 * t4738 * t5418;
    (t16683, t16687, t16692, t16696, t16698, t16700)
}

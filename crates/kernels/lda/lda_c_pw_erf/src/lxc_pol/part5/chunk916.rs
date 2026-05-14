//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 916/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk916<F: Float>(t211: F, t514: F, t6844: F, t1446: F, t6233: F, t3802: F, t519: F, t6460: F, t2171: F, t5234: F, t5238: F, t4738: F, t5409: F, t1318: F, t3854: F, t6404: F) -> (F, F, F, F, F, F, F) {
    let t18390 = t211 * t514 * t6844;
    let t18404 = t1446 * t6233;
    let t18407 = t519 * t3802 * t6460;
    let t18409 = t2171 * t5234;
    let t18413 = t2171 * t5238;
    let t18415 = t4738 * t5409;
    let t18435 = t1318 * t3854 * t6404;
    (t18390, t18404, t18407, t18409, t18413, t18415, t18435)
}

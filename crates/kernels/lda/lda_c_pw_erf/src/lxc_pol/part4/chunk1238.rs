//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1238/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1238<F: Float>(t12501: F, t813: F, t1284: F, t6867: F, t1508: F, t2499: F, t14075: F, t1322: F, t15582: F, t1446: F, t6428: F, t6233: F, t3802: F, t519: F, t6460: F, t2171: F, t5234: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18394 = 8.0 / 15.0 * t12501 * t813;
    let t18396 = 8.0 / 15.0 * t1284 * t6867;
    let t18398 = 2.0 / 15.0 * t1508 * t2499;
    let t18399 = 32.0 / 45.0 * t14075;
    let t18401 = 16.0 / 45.0 * t15582 * t1322;
    let t18403 = 16.0 / 45.0 * t1446 * t6428;
    let t18404 = t1446 * t6233;
    let t18405 = 32.0 / 135.0 * t18404;
    let t18407 = t519 * t3802 * t6460;
    let t18408 = 32.0 / 135.0 * t18407;
    let t18409 = t2171 * t5234;
    (t18394, t18396, t18398, t18399, t18401, t18403, t18405, t18408, t18409)
}

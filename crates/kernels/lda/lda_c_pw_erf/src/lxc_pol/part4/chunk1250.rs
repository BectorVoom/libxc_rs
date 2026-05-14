//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1250/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1250<F: Float>(t3899: F, t571: F, t6194: F, t2171: F, t4938: F, t1325: F, t1390: F, t1392: F, t1440: F, t2328: F, t4738: F, t4946: F, t1278: F, t519: F, t7002: F, t2540: F, t3742: F) -> (F, F, F, F, F, F) {
    let t18575 = t571 * t3899 * t6194;
    let t18576 = 16.0 / 45.0 * t18575;
    let t18578 = 8.0 / 5.0 * t2171 * t4938;
    let t18583 = 8.0 / 15.0 * t1325 * t1440 * t1390 * t2328 * t1392;
    let t18584 = t4738 * t4946;
    let t18585 = 32.0 / 45.0 * t18584;
    let t18589 = 4.0 / 15.0 * t519 * t1440 * t7002 * t1278;
    let t18591 = 8.0 / 45.0 * t3742 * t2540;
    (t18576, t18578, t18583, t18585, t18589, t18591)
}

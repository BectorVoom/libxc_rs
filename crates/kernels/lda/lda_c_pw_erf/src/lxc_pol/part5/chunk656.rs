//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 656/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk656<F: Float>(t2328: F, t504: F, t348: F, t1326: F, t1325: F, t1997: F, t2171: F, t2466: F, t558: F, t352: F, t3867: F, t571: F, t2478: F, t1319: F, t1318: F, t2497: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6263 = t2328 * t504;
    let t6264 = t6263 * t348;
    let t6265 = t1326 * t6264;
    let t6267 = 8.0 / 45.0 * t1325 * t6265;
    let t6269 = 8.0 / 45.0 * t2171 * t1997;
    let t6270 = t2466 * t558;
    let t6271 = t6270 * t352;
    let t6272 = t3867 * t6271;
    let t6274 = 8.0 / 45.0 * t571 * t6272;
    let t6275 = t2478 * t558;
    let t6276 = t6275 * t352;
    let t6277 = t1319 * t6276;
    let t6279 = 8.0 / 45.0 * t1318 * t6277;
    let t6280 = t2497 * t504;
    (t6263, t6264, t6265, t6267, t6269, t6270, t6271, t6272, t6274, t6275, t6276, t6277, t6279, t6280)
}

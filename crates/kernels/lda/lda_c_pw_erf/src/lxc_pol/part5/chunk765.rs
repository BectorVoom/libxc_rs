//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 765/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk765<F: Float>(t1313: F, t7691: F, t519: F, t1326: F, t7651: F, t1991: F, t7643: F, t2429: F, t784: F, t1325: F, t806: F, t3402: F, t2419: F, t833: F, t1308: F, t571: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7692 = t1313 * t7691;
    let t7694 = 4.0 / 15.0 * t519 * t7692;
    let t7695 = t1326 * t7651;
    let t7697 = 8.0 / 15.0 * t519 * t7695;
    let t7698 = t1991 * t7643;
    let t7700 = 4.0 / 9.0 * t519 * t7698;
    let t7701 = t2429 * t784;
    let t7702 = t1991 * t7701;
    let t7704 = 8.0 / 9.0 * t1325 * t7702;
    let t7705 = t2429 * t806;
    let t7706 = t3402 * t7705;
    let t7708 = 4.0 / 9.0 * t519 * t7706;
    let t7709 = t2419 * t833;
    let t7710 = t1308 * t7709;
    let t7712 = 4.0 / 15.0 * t571 * t7710;
    (t7692, t7694, t7695, t7697, t7698, t7700, t7701, t7702, t7704, t7705, t7706, t7708, t7709, t7710, t7712)
}

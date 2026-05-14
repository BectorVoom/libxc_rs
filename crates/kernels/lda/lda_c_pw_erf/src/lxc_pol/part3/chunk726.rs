//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 726/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk726<F: Float>(t494: F, t806: F, t542: F, t5289: F, t1325: F, t1392: F, t789: F, t3806: F, t519: F, t1326: F, t4628: F, t2022: F, t3863: F, t571: F, t1333: F, t833: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5290 = t806 * t494;
    let t5291 = t5290 * t542;
    let t5292 = t5289 * t5291;
    let t5294 = 16.0 / 15.0 * t1325 * t5292;
    let t5295 = t789 * t1392;
    let t5296 = t3806 * t5295;
    let t5298 = 8.0 / 45.0 * t519 * t5296;
    let t5299 = t1326 * t4628;
    let t5301 = 8.0 / 15.0 * t519 * t5299;
    let t5302 = t3863 * t2022;
    let t5304 = 16.0 / 135.0 * t571 * t5302;
    let t5305 = t833 * t1333;
    (t5290, t5291, t5292, t5294, t5295, t5296, t5298, t5299, t5301, t5302, t5304, t5305)
}

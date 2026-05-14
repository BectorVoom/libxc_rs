//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 808/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk808<F: Float>(t146: F, t164: F, t9712: F, t9501: F, t1980: F, t604: F, t223: F, t5210: F, t1179: F, t161: F, t165: F, t177: F, t3279: F, t464: F, t1450: F, t1600: F) -> (F, F, F, F, F, F, F) {
    let t9981 = 0.10864197530864197 * t146 * t9712 * t164;
    let t9986 = 0.3732469135802469 * t9501;
    let t10079 = t604 * t1980;
    let t10082 = 56.0 / 1215.0 * t223 * t5210;
    let t10134 = 28.0 / 1215.0 * t161 * t1179 * t165 * t177;
    let t10148 = t3279 * t464;
    let t10152 = t1450 * t1600;
    (t9981, t9986, t10079, t10082, t10134, t10148, t10152)
}

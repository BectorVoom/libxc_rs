//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 989/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk989<F: Float>(t1696: F, t1798: F, t208: F, t213: F, t4075: F, t794: F, t5374: F, t588: F, t97: F, t3295: F, t802: F, t3035: F, t10190: F, t10196: F, t1966: F, t1967: F, t3441: F, t439: F) -> (F, F, F, F, F, F, F, F) {
    let t13444 = t1798 * t1696 * t208 * t213;
    let t13447 = t794 * t4075 * t208 * t213;
    let t13450 = t5374 * t97 * t588;
    let t13452 = t802 * t3295;
    let t13453 = 2.0 / 15.0 * t13452;
    let t13455 = t802 * t3035 / 5.0;
    let t13456 = 2.0 / 45.0 * t10190;
    let t13457 = 2.0 / 27.0 * t10196;
    let t13461 = t439 * t1966 * t1967 * t3441 / 15.0;
    (t13444, t13447, t13450, t13453, t13455, t13456, t13457, t13461)
}

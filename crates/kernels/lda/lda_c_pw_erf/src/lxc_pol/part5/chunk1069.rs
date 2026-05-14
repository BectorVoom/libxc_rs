//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1069/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1069<F: Float>(t20711: F, t593: F, t13122: F, t4506: F, t13966: F, t20712: F, t13812: F, t13771: F, t20716: F, t4515: F, t6845: F, t822: F, t1960: F, t2528: F, t18023: F, t22222: F, t22225: F, t22228: F, t22231: F, t22234: F, t22237: F, t22239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t22240 = t20711 * t593;
    let t22243 = 16.0 / 15.0 * t4506 * t13122 * t22240;
    let t22246 = 8.0 / 5.0 * t4506 * t13966 * t20712;
    let t22249 = 8.0 / 3.0 * t4506 * t13812 * t20712;
    let t22252 = 32.0 / 15.0 * t13771 * t4515 * t20716;
    let t22254 = 2.0 / 5.0 * t822 * t6845;
    let t22256 = 2.0 / 5.0 * t1960 * t2528;
    let t22257 = 8.0 / 15.0 * t18023;
    let t22258 = -t22222 - t22225 + t22228 + t22231 - t22234 + t22237 - t22239 - t22243 - t22246 + t22249 - t22252 - t22254 - t22256 - t22257;
    (t22240, t22243, t22246, t22249, t22252, t22254, t22256, t22257, t22258)
}

//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 776/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk776<F: Float>(t3330: F, t9418: F, t3418: F, t7522: F, t3421: F, t1736: F, t291: F, t7949: F, t959: F, t9695: F, t3368: F, t3371: F, t277: F, t5294: F, t3694: F, t3439: F) -> (F, F, F, F, F, F, F) {
    let t9944 = t9418 * t3330;
    let t9946 = t3418 * t7522;
    let t9948 = t3421 * t7522;
    let t9950 = t1736 * t291;
    let t9952 = t9950 * t959 * t7949;
    let t9953 = t9695 * t9952;
    let t9955 = t3371 * t3368;
    let t9957 = t277 * t5294;
    let t9958 = t3694 * t291;
    let t9959 = t9958 * t3439;
    (t9944, t9946, t9948, t9953, t9955, t9957, t9959)
}

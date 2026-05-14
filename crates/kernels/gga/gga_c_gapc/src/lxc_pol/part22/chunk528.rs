//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 528/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk528<F: Float>(t1058: F, t3209: F, t761: F, t996: F, t825: F, t932: F, t493: F, t787: F, t1055: F, t773: F, t2206: F, t277: F, t2920: F, t2536: F, t2923: F, t329: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3210 = t3209 * t1058;
    let t3212 = t996 * t761;
    let t3213 = t3212 * t1058;
    let t3216 = t932 * t825;
    let t3217 = t996 * t3216;
    let t3218 = t493 * t787;
    let t3219 = t3217 * t3218;
    let t3221 = t1055 * t773;
    let t3222 = t3209 * t3221;
    let t3224 = t277 * t2206;
    let t3225 = t2920 * t3224;
    let t3227 = t2923 * t329 * t2536;
    (t3210, t3212, t3213, t3216, t3217, t3218, t3219, t3221, t3222, t3224, t3225, t3227)
}

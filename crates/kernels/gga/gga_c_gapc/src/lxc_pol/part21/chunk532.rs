//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 532/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk532<F: Float>(t2554: F, t3288: F, t1092: F, t190: F, t2206: F, t1453: F, t134: F, t329: F, t314: F, t154: F) -> (F, F, F, F, F, F, F) {
    let t3289 = t3288 * t2554;
    let t3290 = t1092 * t3289;
    let t3292 = t2206 * t190;
    let t3293 = t3292 * t1453;
    let t3295 = t134 * t329;
    let t3296 = t3295 * t314;
    let t3297 = t154 * t3296;
    (t3289, t3290, t3292, t3293, t3295, t3296, t3297)
}

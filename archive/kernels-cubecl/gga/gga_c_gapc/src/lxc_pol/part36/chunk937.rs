//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 937/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk937<F: Float>(t203: F, t5700: F, t674: F, t11399: F, t11398: F, t3663: F, t561: F, t3665: F, t1453: F, t3673: F, t169: F, t8951: F) -> (F, F, F, F, F, F, F, F) {
    let t11401 = t5700 * t674 * t203;
    let t11402 = t11399 * t11401;
    let t11403 = t11398 * t11402;
    let t11405 = t561 * t3663;
    let t11406 = t11405 * t3665;
    let t11408 = t3673 * t1453;
    let t11409 = t169 * t11408;
    let t11410 = t11409 * t8951;
    (t11401, t11402, t11403, t11405, t11406, t11408, t11409, t11410)
}

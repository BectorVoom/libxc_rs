//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 553/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk553<F: Float>(t3139: F, t3144: F, t1013: F, t608: F, t1016: F, t1019: F, t561: F, t1457: F, t190: F, t1453: F, t134: F, t200: F) -> (F, F, F, F, F, F) {
    let t3145 = t3139 * t3144;
    let t3147 = t1013 * t608;
    let t3150 = t561 * t1016 * t1019;
    let t3152 = t1457 * t190;
    let t3153 = t3152 * t1453;
    let t3155 = t134 * t200;
    (t3145, t3147, t3150, t3152, t3153, t3155)
}

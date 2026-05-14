//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 855/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk855<F: Float>(t4961: F, t8002: F, t3623: F, t17148: F, t8129: F, t3927: F, t4776: F, t2813: F, t14617: F, t14619: F, t16998: F, t17135: F, t17138: F, t17142: F, t17145: F, t17149: F, t17152: F, t17155: F, t2668: F, t2721: F, t2812: F, t3907: F, t3917: F, t8107: F, t8114: F, t8127: F, t8214: F, t930: F, t953: F) -> (F, F, F) {
    let t17160 = t8002 * t4961;
    let t17161 = t3623 * t17160;
    let t17164 = t17148 * t8129;
    let t17169 = t4776 * t3927;
    let t17170 = t2813 * t17169;
    let t17173 = 0.15146801702008125515e1 * t2721 * t17135 + t8107 - 0.34014423178468276541e6 * t8214 * t17138 + 0.26372962023724310886e4 * t3917 * t17142 - 0.23229342182245570105e2 * t2668 * t17145 + 0.99866506516985762611e3 * t8114 * t17149 + 0.17386322979577515709e0 * t930 * t17152 - 0.23181763972770020946e0 * t930 * t17155 + 0.30228422675018518374e-1 * t953 * t16998 + 0.69688026546736710315e2 * t3907 * t17161 + 0.11983980782038291513e5 * t8127 * t17164 + 0.10076140891672839458e-1 * t14617 - 0.20152281783345678915e-1 * t14619 + 0.1169609647897054359e2 * t2812 * t17170;
    (t17160, t17169, t17173)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 913/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk913<F: Float>(t14617: F, t14619: F, t16998: F, t17135: F, t17138: F, t17142: F, t17145: F, t17149: F, t17152: F, t17155: F, t17161: F, t17164: F, t17170: F, t2668: F, t2721: F, t2812: F, t3907: F, t3917: F, t8107: F, t8114: F, t8127: F, t8214: F, t930: F, t953: F) -> F {
    let t17173 = F::cast_from(0.15146801702008125515e1_f64) * t2721 * t17135 + t8107 - F::cast_from(0.34014423178468276541e6_f64) * t8214 * t17138 + F::cast_from(0.26372962023724310886e4_f64) * t3917 * t17142 - F::cast_from(0.23229342182245570105e2_f64) * t2668 * t17145 + F::cast_from(0.99866506516985762611e3_f64) * t8114 * t17149 + F::cast_from(0.17386322979577515709e0_f64) * t930 * t17152 - F::cast_from(0.23181763972770020946e0_f64) * t930 * t17155 + F::cast_from(0.30228422675018518374e-1_f64) * t953 * t16998 + F::cast_from(0.69688026546736710315e2_f64) * t3907 * t17161 + F::cast_from(0.11983980782038291513e5_f64) * t8127 * t17164 + F::cast_from(0.10076140891672839458e-1_f64) * t14617 - F::cast_from(0.20152281783345678915e-1_f64) * t14619 + F::cast_from(0.1169609647897054359e2_f64) * t2812 * t17170;
    t17173
}

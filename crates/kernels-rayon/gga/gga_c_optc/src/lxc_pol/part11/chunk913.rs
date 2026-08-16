//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 913/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk913(t14617: f64, t14619: f64, t16998: f64, t17135: f64, t17138: f64, t17142: f64, t17145: f64, t17149: f64, t17152: f64, t17155: f64, t17161: f64, t17164: f64, t17170: f64, t2668: f64, t2721: f64, t2812: f64, t3907: f64, t3917: f64, t8107: f64, t8114: f64, t8127: f64, t8214: f64, t930: f64, t953: f64) -> f64 {
    let t17173 = 0.15146801702008125515e1_f64 * t2721 * t17135 + t8107 - 0.34014423178468276541e6_f64 * t8214 * t17138 + 0.26372962023724310886e4_f64 * t3917 * t17142 - 0.23229342182245570105e2_f64 * t2668 * t17145 + 0.99866506516985762611e3_f64 * t8114 * t17149 + 0.17386322979577515709e0_f64 * t930 * t17152 - 0.23181763972770020946e0_f64 * t930 * t17155 + 0.30228422675018518374e-1_f64 * t953 * t16998 + 0.69688026546736710315e2_f64 * t3907 * t17161 + 0.11983980782038291513e5_f64 * t8127 * t17164 + 0.10076140891672839458e-1_f64 * t14617 - 0.20152281783345678915e-1_f64 * t14619 + 0.1169609647897054359e2_f64 * t2812 * t17170;
    t17173
}

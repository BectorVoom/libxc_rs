//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 630/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk630<F: Float>(t3237: F, t3245: F, t1115: F, t2367: F, t1162: F, t3097: F, t914: F, t3088: F, t1172: F, t2586: F, t1170: F, t1152: F, t1150: F, t1179: F, t3148: F, t3153: F, t3157: F, t3175: F, t3186: F, t3189: F, t3192: F, t3195: F, t3199: F, t3203: F, t3206: F, t3212: F, t3214: F, t3217: F, t3220: F, t3225: F, t3230: F, t3234: F, t3238: F, t3244: F) -> (F,) {
    let t3246 = t3245 * t3237;
    let t3249 = t2367 * t1115;
    let t3250 = t1162 * t3249;
    let t3252 = t914 * t3097;
    let t3255 = t914 * t3088;
    let t3258 = t2586 * t1172;
    let t3259 = t1170 * t3258;
    let t3261 = t2367 * t1152;
    let t3262 = t1150 * t3261;
    let t3264 = 0.6717427261115226305e-2 * t3175 + 0.50380704458364197288e-2 * t1179 * t3157 + 0.83967840763940328814e-2 * t1179 * t3148 + 0.23229342182245570105e2 * t3186 * t3189 - 0.77431140607485233683e1 * t3192 * t3195 - t3199 - t3203 + 0.5848048239485271795e1 * t1170 * t3206 + 0.8790987341241436962e3 * t3212 * t3214 - 0.4395493670620718481e3 * t3217 * t3220 + 0.11360101276506094136e1 * t1150 * t3225 - 0.10076140891672839458e-1 * t1179 * t3153 - 0.57954409931925052364e-1 * t1162 * t3230 + 0.779739765264702906e1 * t3234 * t3238 + 0.75734008510040627574e0 * t3244 * t3246 + 0.19318136643975017455e-1 * t3250 + 0.28977204965962526182e-1 * t1162 * t3252 + 0.38636273287950034909e-1 * t1162 * t3255 + 0.779739765264702906e1 * t3259 + 0.75734008510040627574e0 * t3262;
    (t3264,)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 647/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk647(t1150: f64, t3261: f64, t1162: f64, t1170: f64, t1179: f64, t3148: f64, t3153: f64, t3157: f64, t3175: f64, t3186: f64, t3189: f64, t3192: f64, t3195: f64, t3199: f64, t3203: f64, t3206: f64, t3212: f64, t3214: f64, t3217: f64, t3220: f64, t3225: f64, t3230: f64, t3234: f64, t3238: f64, t3244: f64, t3246: f64, t3250: f64, t3252: f64, t3255: f64, t3259: f64) -> f64 {
    let t3262 = t1150 * t3261;
    let t3264 = 0.6717427261115226305e-2_f64 * t3175 + 0.50380704458364197288e-2_f64 * t1179 * t3157 + 0.83967840763940328814e-2_f64 * t1179 * t3148 + 0.23229342182245570105e2_f64 * t3186 * t3189 - 0.77431140607485233683e1_f64 * t3192 * t3195 - t3199 - t3203 + 0.5848048239485271795e1_f64 * t1170 * t3206 + 0.8790987341241436962e3_f64 * t3212 * t3214 - 0.4395493670620718481e3_f64 * t3217 * t3220 + 0.11360101276506094136e1_f64 * t1150 * t3225 - 0.10076140891672839458e-1_f64 * t1179 * t3153 - 0.57954409931925052364e-1_f64 * t1162 * t3230 + 0.779739765264702906e1_f64 * t3234 * t3238 + 0.75734008510040627574e0_f64 * t3244 * t3246 + 0.19318136643975017455e-1_f64 * t3250 + 0.28977204965962526182e-1_f64 * t1162 * t3252 + 0.38636273287950034909e-1_f64 * t1162 * t3255 + 0.779739765264702906e1_f64 * t3259 + 0.75734008510040627574e0_f64 * t3262;
    t3264
}

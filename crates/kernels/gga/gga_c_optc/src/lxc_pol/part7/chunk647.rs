//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 647/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk647<F: Float>(t1150: F, t3261: F, t1162: F, t1170: F, t1179: F, t3148: F, t3153: F, t3157: F, t3175: F, t3186: F, t3189: F, t3192: F, t3195: F, t3199: F, t3203: F, t3206: F, t3212: F, t3214: F, t3217: F, t3220: F, t3225: F, t3230: F, t3234: F, t3238: F, t3244: F, t3246: F, t3250: F, t3252: F, t3255: F, t3259: F) -> F {
    let t3262 = t1150 * t3261;
    let t3264 = F::cast_from(0.6717427261115226305e-2_f64) * t3175 + F::cast_from(0.50380704458364197288e-2_f64) * t1179 * t3157 + F::cast_from(0.83967840763940328814e-2_f64) * t1179 * t3148 + F::cast_from(0.23229342182245570105e2_f64) * t3186 * t3189 - F::cast_from(0.77431140607485233683e1_f64) * t3192 * t3195 - t3199 - t3203 + F::cast_from(0.5848048239485271795e1_f64) * t1170 * t3206 + F::cast_from(0.8790987341241436962e3_f64) * t3212 * t3214 - F::cast_from(0.4395493670620718481e3_f64) * t3217 * t3220 + F::cast_from(0.11360101276506094136e1_f64) * t1150 * t3225 - F::cast_from(0.10076140891672839458e-1_f64) * t1179 * t3153 - F::cast_from(0.57954409931925052364e-1_f64) * t1162 * t3230 + F::cast_from(0.779739765264702906e1_f64) * t3234 * t3238 + F::cast_from(0.75734008510040627574e0_f64) * t3244 * t3246 + F::cast_from(0.19318136643975017455e-1_f64) * t3250 + F::cast_from(0.28977204965962526182e-1_f64) * t1162 * t3252 + F::cast_from(0.38636273287950034909e-1_f64) * t1162 * t3255 + F::cast_from(0.779739765264702906e1_f64) * t3259 + F::cast_from(0.75734008510040627574e0_f64) * t3262;
    t3264
}

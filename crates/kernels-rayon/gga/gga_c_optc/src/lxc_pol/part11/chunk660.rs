//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 660/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk660(t1: f64, t438: f64, t5328: f64, t450: f64, t464: f64, t5255: f64, t155: f64, t146: f64, t455: f64, t5274: f64, t1150: f64, t1170: f64, t1529: f64, t1541: f64, t3186: f64, t3192: f64, t3199: f64, t3203: f64, t3212: f64, t3217: f64, t4492: f64, t4501: f64, t451: f64, t4515: f64, t4517: f64, t459: f64, t5389: f64, t5394: f64, t5399: f64, t5404: f64, t5408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5412 = t5328 * t1 * t438;
    let t5413 = t450 * t5412;
    let t5416 = t464 * t5255;
    let t5417 = t155 * t5416;
    let t5421 = t146 * t455 * t5274;
    let t5429 = -t3199 - t3203 - 0.5373941808892181044e-1_f64 * t4515 + 0.5848048239485271795e1_f64 * t1170 * t5389 + 0.8790987341241436962e3_f64 * t3212 * t5394 - 0.4395493670620718481e3_f64 * t3217 * t5399 + 0.11360101276506094136e1_f64 * t1150 * t5404 + 0.23229342182245570105e2_f64 * t3186 * t5408 - 0.77431140607485233683e1_f64 * t3192 * t5413 + 0.25526223592237859959e0_f64 * t5417 * t451 + 0.84999801233490076802e0_f64 * t5421 * t459 - 0.6237918122117623248e2_f64 * t4492 * t1541 - 0.60587206808032502059e1_f64 * t4501 * t1529 - 0.15454509315180013964e0_f64 * t4517;
    (t5412, t5413, t5416, t5417, t5421, t5429)
}

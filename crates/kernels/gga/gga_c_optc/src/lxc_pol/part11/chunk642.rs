//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 642/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk642<F: Float>(t1150: F, t1170: F, t1529: F, t1541: F, t3186: F, t3192: F, t3199: F, t3203: F, t3212: F, t3217: F, t4492: F, t4501: F, t451: F, t4515: F, t4517: F, t459: F, t5389: F, t5394: F, t5399: F, t5404: F, t5408: F, t5413: F, t5417: F, t5421: F) -> (F,) {
    let t5429 = -t3199 - t3203 - 0.5373941808892181044e-1 * t4515 + 0.5848048239485271795e1 * t1170 * t5389 + 0.8790987341241436962e3 * t3212 * t5394 - 0.4395493670620718481e3 * t3217 * t5399 + 0.11360101276506094136e1 * t1150 * t5404 + 0.23229342182245570105e2 * t3186 * t5408 - 0.77431140607485233683e1 * t3192 * t5413 + 0.25526223592237859959e0 * t5417 * t451 + 0.84999801233490076802e0 * t5421 * t459 - 0.6237918122117623248e2 * t4492 * t1541 - 0.60587206808032502059e1 * t4501 * t1529 - 0.15454509315180013964e0 * t4517;
    (t5429,)
}

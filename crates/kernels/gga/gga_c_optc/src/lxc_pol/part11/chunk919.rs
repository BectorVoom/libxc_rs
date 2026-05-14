//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 919/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk919<F: Float>(t15786: F, t17921: F, t15274: F, t18023: F, t3087: F, t914: F, t1162: F, t1179: F, t12812: F, t1536: F, t15911: F, t15978: F, t15981: F, t15984: F, t17720: F, t18114: F, t3234: F, t4444: F, t4457: F, t4464: F, t5298: F) -> (F, F, F, F, F) {
    let t18117 = t15786 * t17921;
    let t18120 = t15786 * t15274;
    let t18130 = t3087 * t18023;
    let t18131 = t914 * t18130;
    let t18137 = -0.2339219295794108718e2 * t3234 * t18114 + 0.8790987341241436962e3 * t4457 * t18117 - 0.4395493670620718481e3 * t4464 * t18120 + 0.84999801233490076802e0 * t15911 * t1536 - 0.8060912713338271566e-1 * t4444 * t5298 + 0.50380704458364197288e-2 * t1179 * t17720 + 0.2686970904446090522e-1 * t12812 + 0.11590881986385010473e0 * t1162 * t18131 + 0.51052447184475719918e0 * t15978 + 0.11360101276506094136e1 * t15981 - 0.4395493670620718481e3 * t15984;
    (t18117, t18120, t18130, t18131, t18137)
}

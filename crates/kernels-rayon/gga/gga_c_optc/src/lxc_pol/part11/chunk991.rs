//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 991/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk991(t1162: f64, t1179: f64, t12812: f64, t1536: f64, t15911: f64, t15978: f64, t15981: f64, t15984: f64, t17720: f64, t18114: f64, t18117: f64, t18120: f64, t18131: f64, t3234: f64, t4444: f64, t4457: f64, t4464: f64, t5298: f64) -> f64 {
    let t18137 = -0.2339219295794108718e2_f64 * t3234 * t18114 + 0.8790987341241436962e3_f64 * t4457 * t18117 - 0.4395493670620718481e3_f64 * t4464 * t18120 + 0.84999801233490076802e0_f64 * t15911 * t1536 - 0.8060912713338271566e-1_f64 * t4444 * t5298 + 0.50380704458364197288e-2_f64 * t1179 * t17720 + 0.2686970904446090522e-1_f64 * t12812 + 0.11590881986385010473e0_f64 * t1162 * t18131 + 0.51052447184475719918e0_f64 * t15978 + 0.11360101276506094136e1_f64 * t15981 - 0.4395493670620718481e3_f64 * t15984;
    t18137
}

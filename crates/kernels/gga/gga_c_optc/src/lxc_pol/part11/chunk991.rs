//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 991/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk991<F: Float>(t1162: F, t1179: F, t12812: F, t1536: F, t15911: F, t15978: F, t15981: F, t15984: F, t17720: F, t18114: F, t18117: F, t18120: F, t18131: F, t3234: F, t4444: F, t4457: F, t4464: F, t5298: F) -> F {
    let t18137 = -F::cast_from(0.2339219295794108718e2_f64) * t3234 * t18114 + F::cast_from(0.8790987341241436962e3_f64) * t4457 * t18117 - F::cast_from(0.4395493670620718481e3_f64) * t4464 * t18120 + F::cast_from(0.84999801233490076802e0_f64) * t15911 * t1536 - F::cast_from(0.8060912713338271566e-1_f64) * t4444 * t5298 + F::cast_from(0.50380704458364197288e-2_f64) * t1179 * t17720 + F::cast_from(0.2686970904446090522e-1_f64) * t12812 + F::cast_from(0.11590881986385010473e0_f64) * t1162 * t18131 + F::cast_from(0.51052447184475719918e0_f64) * t15978 + F::cast_from(0.11360101276506094136e1_f64) * t15981 - F::cast_from(0.4395493670620718481e3_f64) * t15984;
    t18137
}

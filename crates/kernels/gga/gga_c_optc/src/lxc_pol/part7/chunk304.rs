//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 304/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk304<F: Float>(t314: F, t324: F, t899: F, t913: F, t917: F, t921: F, t927: F, t930: F, t931: F, t940: F, t943: F, t947: F, t951: F, t953: F) -> F {
    let t956 = F::cast_from(0.11360101276506094136e1_f64) * t913 * t917 - F::cast_from(0.23181763972770020946e0_f64) * t921 * t324 + t927 + F::cast_from(0.28977204965962526182e-1_f64) * t930 * t931 + F::cast_from(0.5848048239485271795e1_f64) * t940 * t943 - F::cast_from(0.4030456356669135783e-1_f64) * t947 * t314 + t951 + F::cast_from(0.50380704458364197288e-2_f64) * t953 * t899;
    t956
}

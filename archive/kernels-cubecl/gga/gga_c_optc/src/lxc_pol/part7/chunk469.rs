//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 469/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk469<F: Float>(t2257: F, t2280: F, t2259: F, t2266: F, t2272: F, t2276: F, t2282: F, t2286: F, t2289: F, t2292: F) -> F {
    let t2305 = F::cast_from(0.96922222222222222222e3_f64) * t2257;
    let t2310 = F::cast_from(0.13111111111111111111e3_f64) * t2280;
    let t2315 = t2305 + F::cast_from(0.48461111111111111112e3_f64) * t2259 - F::cast_from(0.48461111111111111111e3_f64) * t2266 + F::cast_from(0.14538333333333333333e4_f64) * t2272 - F::cast_from(0.72691666666666666667e3_f64) * t2276 + t2310 + F::cast_from(0.10488888888888888889e3_f64) * t2282 - F::cast_from(0.26222222222222222222e2_f64) * t2286 + F::cast_from(0.15733333333333333333e3_f64) * t2289 - F::cast_from(0.78666666666666666667e2_f64) * t2292;
    t2315
}

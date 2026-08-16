//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 467/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk467<F: Float>(t2258: F, t2259: F, t2266: F, t2272: F, t2276: F, t2281: F, t2282: F, t2286: F, t2289: F, t2292: F, t970: F, t973: F) -> (F, F) {
    let t2294 = t2258 + F::cast_from(0.12925555555555555555e1_f64) * t2259 - F::cast_from(0.12925555555555555555e1_f64) * t2266 + F::cast_from(0.38776666666666666666e1_f64) * t2272 - F::cast_from(0.19388333333333333333e1_f64) * t2276 + t2281 + F::cast_from(0.1642e-2_f64) * t2282 - F::cast_from(0.4105e-3_f64) * t2286 + F::cast_from(0.2463e-2_f64) * t2289 - F::cast_from(0.12315e-2_f64) * t2292;
    let t2296 = t970 * t973;
    (t2294, t2296)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 489/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk489<F: Float>(t2374: F, t2418: F, t2416: F, t2257: F, t2259: F, t2266: F, t2272: F, t2276: F) -> (F, F, F) {
    let t2419 = t2374 * t2418;
    let t2421 = F::cast_from(0.16081824322151104822e2_f64) * t2416 * t2419;
    let t2422 = F::cast_from(0.12361111111111111111e-1_f64) * t2257;
    let t2427 = t2422 + F::cast_from(0.61805555555555555556e-2_f64) * t2259 - F::cast_from(0.61805555555555555555e-2_f64) * t2266 + F::cast_from(0.18541666666666666667e-1_f64) * t2272 - F::cast_from(0.92708333333333333333e-2_f64) * t2276;
    (t2419, t2421, t2427)
}

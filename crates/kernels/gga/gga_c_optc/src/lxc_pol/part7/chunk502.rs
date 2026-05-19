//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 502/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk502<F: Float>(t2494: F, t818: F, t2257: F, t2280: F, t2259: F, t2266: F, t2272: F, t2276: F, t2282: F, t2286: F, t2289: F, t2292: F, t2384: F, t2392: F, t2400: F, t2402: F) -> (F, F) {
    let t2495 = t2494 * t818;
    let t2500 = F::cast_from(0.68863333333333333333e0_f64) * t2257;
    let t2507 = F::cast_from(0.17365833333333333333e0_f64) * t2280;
    let t2512 = -F::new(0.17648625e1) * t2384 + F::new(0.3529725e1) * t2392 + t2500 + F::cast_from(0.34431666666666666666e0_f64) * t2259 - F::cast_from(0.34431666666666666667e0_f64) * t2266 + F::new(0.103295e1) * t2272 - F::new(0.516475e0) * t2276 + F::new(0.31558125e0) * t2400 + F::new(0.6311625e0) * t2402 + t2507 + F::cast_from(0.13892666666666666667e0_f64) * t2282 - F::cast_from(0.34731666666666666667e-1_f64) * t2286 + F::new(0.20839e0) * t2289 - F::new(0.104195e0) * t2292;
    (t2495, t2512)
}

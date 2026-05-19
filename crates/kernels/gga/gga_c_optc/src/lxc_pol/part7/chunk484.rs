//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 484/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk484<F: Float>(t2391: F, t780: F, t2257: F, t214: F, t2383: F, t787: F, t2280: F, t2259: F, t2266: F, t2272: F, t2276: F, t2282: F, t2286: F, t2289: F, t2292: F, t2384: F) -> (F, F, F, F, F) {
    let t2392 = t780 * t2391;
    let t2394 = F::cast_from(0.39862222222222222223e0_f64) * t2257;
    let t2399 = F::new(1.0)/F::sqrt(t214);
    let t2400 = t2399 * t2383;
    let t2402 = t787 * t2391;
    let t2404 = F::cast_from(0.13692777777777777778e0_f64) * t2280;
    let t2409 = -F::new(0.9494625e0) * t2384 + F::new(0.1898925e1) * t2392 + t2394 + F::cast_from(0.19931111111111111111e0_f64) * t2259 - F::cast_from(0.19931111111111111111e0_f64) * t2266 + F::cast_from(0.59793333333333333334e0_f64) * t2272 - F::cast_from(0.29896666666666666667e0_f64) * t2276 + F::new(0.15358125e0) * t2400 + F::new(0.3071625e0) * t2402 + t2404 + F::cast_from(0.10954222222222222222e0_f64) * t2282 - F::cast_from(0.27385555555555555556e-1_f64) * t2286 + F::cast_from(0.16431333333333333333e0_f64) * t2289 - F::cast_from(0.82156666666666666667e-1_f64) * t2292;
    (t2392, t2399, t2400, t2402, t2409)
}

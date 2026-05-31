//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 495/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk495<F: Float>(t2441: F, t847: F, t2257: F, t2259: F, t2266: F, t2272: F, t2276: F, t232: F, t2280: F, t2282: F, t2286: F, t2289: F, t2292: F, t2384: F, t2392: F, t2400: F, t2402: F) -> (F, F, F, F) {
    let t2443 = F::cast_from(0.11696446794910408142e1_f64) * t2441 * t847;
    let t2444 = F::cast_from(0.23744444444444444444e-1_f64) * t2257;
    let t2449 = t2444 + F::cast_from(0.11872222222222222222e-1_f64) * t2259 - F::cast_from(0.11872222222222222222e-1_f64) * t2266 + F::cast_from(0.35616666666666666666e-1_f64) * t2272 - F::cast_from(0.17808333333333333333e-1_f64) * t2276;
    let t2451 = F::cast_from(0.62182e-1_f64) * t2449 * t232;
    let t2454 = F::cast_from(0.40256666666666666667e0_f64) * t2257;
    let t2461 = F::cast_from(0.137975e0_f64) * t2280;
    let t2466 = -F::cast_from(0.1294625e1_f64) * t2384 + F::cast_from(0.258925e1_f64) * t2392 + t2454 + F::cast_from(0.20128333333333333334e0_f64) * t2259 - F::cast_from(0.20128333333333333333e0_f64) * t2266 + F::cast_from(0.60385e0_f64) * t2272 - F::cast_from(0.301925e0_f64) * t2276 + F::cast_from(0.82524375e-1_f64) * t2400 + F::cast_from(0.16504875e0_f64) * t2402 + t2461 + F::cast_from(0.11038e0_f64) * t2282 - F::cast_from(0.27595e-1_f64) * t2286 + F::cast_from(0.16557e0_f64) * t2289 - F::cast_from(0.82785e-1_f64) * t2292;
    (t2443, t2449, t2451, t2466)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 483/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk483(t2382: f64, t2383: f64, t2257: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64) -> (f64, f64) {
    let t2384 = t2382 * t2383;
    let t2386 = 4.0_f64 / 9.0_f64 * t2257;
    let t2391 = t2386 + 2.0_f64 / 9.0_f64 * t2259 - 2.0_f64 / 9.0_f64 * t2266 + 2.0_f64 / 3.0_f64 * t2272 - t2276 / 3.0_f64;
    (t2384, t2391)
}

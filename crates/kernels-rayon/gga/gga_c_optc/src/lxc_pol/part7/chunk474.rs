//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 474/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk474(t2329: f64, t278: f64, t2328: f64, t2326: f64, t136: f64, t3: f64, t362: f64, t190: f64, t288: f64) -> (f64, f64, f64, f64, f64) {
    let t2330 = t2329 * t278;
    let t2331 = 1.0_f64 / t2330;
    let t2332 = t2328 * t2331;
    let t2333 = t2326 * t2332;
    let t2335 = t136 * t3;
    let t2336 = t2335 * t362;
    let t2337 = t288 * t190 * t2336;
    (t2331, t2332, t2333, t2336, t2337)
}

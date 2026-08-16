//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 558/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk558(t2554: f64, t3288: f64, t1092: f64, t190: f64, t2206: f64, t1453: f64, t134: f64, t329: f64, t314: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3289 = t3288 * t2554;
    let t3290 = t1092 * t3289;
    let t3292 = t2206 * t190;
    let t3293 = t3292 * t1453;
    let t3295 = t134 * t329;
    let t3296 = t3295 * t314;
    let t3297 = t154 * t3296;
    (t3289, t3290, t3292, t3293, t3295, t3296, t3297)
}

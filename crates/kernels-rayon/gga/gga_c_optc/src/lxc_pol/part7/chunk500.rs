//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 500/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk500(t2474: f64, t2476: f64, t845: f64, t2257: f64, t2259: f64, t2266: f64, t2272: f64, t2276: f64, t805: f64, t809: f64, t248: f64, t808: f64) -> (f64, f64, f64, f64, f64) {
    let t2477 = t2474 * t2476;
    let t2479 = 0.17315755899375863299e2_f64 * t845 * t2477;
    let t2480 = 0.22831111111111111111e-1_f64 * t2257;
    let t2485 = t2480 + 0.11415555555555555555e-1_f64 * t2259 - 0.11415555555555555555e-1_f64 * t2266 + 0.34246666666666666666e-1_f64 * t2272 - 0.17123333333333333333e-1_f64 * t2276;
    let t2488 = t805 * t809;
    let t2491 = t808 * t248;
    (t2477, t2479, t2485, t2488, t2491)
}

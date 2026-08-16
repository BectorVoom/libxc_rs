//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 856/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk856(t16393: f64, t7129: f64, t16397: f64, t2126: f64, t10002: f64, t10008: f64, t13482: f64, t13487: f64, t16394: f64, t16398: f64, t2124: f64, t2168: f64, t7091: f64, t7094: f64, t9913: f64, t9915: f64) -> (f64, f64, f64) {
    let t16554 = t7129 * t16393;
    let t16557 = t2126 * t16397;
    let t16566 = 0.18137053605011111023e0_f64 * t2168 * t16398 - 0.90685268025055555117e0_f64 * t2168 * t16394 - 0.15647690681619764138e1_f64 * t2124 * t16554 + 0.52158968938732547127e0_f64 * t2124 * t16557 - t7091 - t7094 - 0.23981215322181357908e1_f64 * t9913 - 0.40568086952347536654e1_f64 * t9915 - 0.24340852171408521993e1_f64 * t13482 - 0.16927916698010370288e1_f64 * t13487 - 0.11990607661090678954e1_f64 * t10002 - 0.20284043476173768327e1_f64 * t10008;
    (t16554, t16557, t16566)
}

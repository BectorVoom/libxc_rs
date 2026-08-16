//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1080/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1080(t188: f64, t202: f64, t23047: f64, t6529: f64, t740: f64, t108: f64, t176: f64, t185: f64, t203: f64, t22052: f64, t22700: f64, t22703: f64, t22705: f64, t22708: f64, t22711: f64, t22713: f64, t22716: f64, t22719: f64) -> f64 {
    let t23431 = 7280.0_f64 / 81.0_f64 * t188 * t23047 * t202;
    let t23432 = t6529 * t740;
    let t23434 = t22700 + t22703 + t22705 + t22708 - t22711 - t22713 - t22716 - t22719 + t176 * t185 * t22052 * t108 * t203 / 2.0_f64 + t23431 - 14.0_f64 * t23432;
    t23434
}

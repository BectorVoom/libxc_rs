//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 359/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk359(t1196: f64, t1200: f64, t1205: f64, t485: f64, t275: f64, t176: f64, t1107: f64, t496: f64, t492: f64, t490: f64, t487: f64, t426: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1207 = t1196 * t485 - t1200 * t1205;
    let t1208 = t1207 * t275;
    let t1210 = t176 * t1208 * sigma2;
    let t1213 = t1107 * t496;
    let t1214 = t492 * t1213;
    let t1216 = t490 * t1214 / 6.0_f64;
    let t1217 = t176 * t487;
    let t1218 = t275 * sigma2;
    let t1219 = t1218 * t426;
    (t1207, t1210, t1214, t1216, t1217, t1218, t1219)
}

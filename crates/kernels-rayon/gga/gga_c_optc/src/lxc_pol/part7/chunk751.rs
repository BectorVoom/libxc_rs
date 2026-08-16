//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 751/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk751(t1000: f64, t7244: f64, t914: f64, t1015: f64, t2337: f64, t2360: f64, t2364: f64, t2433: f64, t2544: f64, t355: f64, t4038: f64, t7175: f64, t7180: f64, t7183: f64, t7186: f64, t7188: f64, t7195: f64, t7199: f64, t7204: f64, t7208: f64, t7210: f64, t7215: f64, t7219: f64, t7224: f64, t7230: f64, t7235: f64, t7240: f64, t999: f64) -> (f64, f64, f64) {
    let t7245 = t1000 * t7244;
    let t7246 = t914 * t7245;
    let t7249 = -100.0_f64 / 27.0_f64 * t2433 * t7175 - t4038 * t7180 + 50.0_f64 / 9.0_f64 * t7183 * t1015 - 50.0_f64 / 3.0_f64 * t7186 - 50.0_f64 * t7188 * t1015 - 616.0_f64 / 27.0_f64 * t355 * t7195 + 44.0_f64 / 9.0_f64 * t7199 + t7204 - 100.0_f64 / 81.0_f64 * t7208 + 100.0_f64 / 27.0_f64 * t7210 + 100.0_f64 / 81.0_f64 * t7215 + 20000.0_f64 / 81.0_f64 * t7219 * t7224 - 380000.0_f64 / 81.0_f64 * t7230 * t2337 + 20000.0_f64 / 81.0_f64 * t7235 - 4.0_f64 / 3.0_f64 * t2364 * t2544 + t7240 / 6.0_f64 + t2360 * t2544 / 2.0_f64 + t999 * t7246 / 6.0_f64;
    (t7245, t7246, t7249)
}

//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 763/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk763(t1008: f64, t7312: f64, t1007: f64, t2466: f64, t2472: f64, t3802: f64, t845: f64, t1002: f64, t1015: f64, t2337: f64, t2360: f64, t2364: f64, t2551: f64, t2563: f64, t2569: f64, t2822: f64, t3980: f64, t7259: f64, t7263: f64, t7268: f64, t7276: f64, t7279: f64, t7281: f64, t7285: f64, t7288: f64, t7295: f64, t7301: f64, t7304: f64, t7308: f64, t960: f64, t999: f64) -> (f64, f64, f64, f64, f64) {
    let t7313 = t1008 * t7312;
    let t7314 = t1007 * t7313;
    let t7318 = t2472 * t2466 * t3802;
    let t7320 = 0.51947267698127589897e2_f64 * t845 * t7318;
    let t7321 = 2.0_f64 / 3.0_f64 * t2360 * t2551 + 14.0_f64 / 27.0_f64 * t999 * t7259 + t7263 * t1002 / 2.0_f64 + t999 * t7268 - 0.77534644304710291488e-2_f64 * t3980 * t960 * t2569 * t2822 - t7276 / 9.0_f64 - t7279 / 3.0_f64 + t7281 / 3.0_f64 - t2360 * t2563 + 44.0_f64 / 9.0_f64 * t7285 * t1002 - 8.0_f64 / 9.0_f64 * t7288 + 8.0_f64 / 3.0_f64 * t2364 * t2563 - 16.0_f64 / 9.0_f64 * t2364 * t2551 + 2.0_f64 / 9.0_f64 * t7295 - 4.0_f64 / 3.0_f64 * t999 * t7301 - 8.0_f64 / 3.0_f64 * t7304 * t1002 + 20000.0_f64 / 27.0_f64 * t7308 * t2337 + 34100.0_f64 / 243.0_f64 * t7314 * t1015 - t7320;
    (t7313, t7314, t7318, t7320, t7321)
}

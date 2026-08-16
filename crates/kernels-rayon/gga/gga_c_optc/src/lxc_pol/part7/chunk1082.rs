//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1082/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1082(t2559: f64, t7337: f64, t2359: f64, t7284: f64, t2409: f64, t2419: f64, t7681: f64, t2375: f64, t2416: f64, t2364: f64, t7275: f64, t195: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23459 = 0.70178680769462448852e1_f64 * t7337 * t2559;
    let t23460 = t2359 * t7284;
    let t23465 = 0.57894567559743977359e3_f64 * t7681 * t2419 * t2409;
    let t23468 = 36.0_f64 * t2416 * t2375 * t2409;
    let t23469 = t2364 * t7275;
    let t23471 = t6 * t195;
    (t23459, t23460, t23465, t23468, t23469, t23471)
}

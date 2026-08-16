//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1217/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1217(t37441: f64, t28540: f64, t1310: f64, t16301: f64, t22290: f64, t22293: f64, t23336: f64, t28530: f64, t48024: f64, t48028: f64, t48040: f64, t55933: f64, t606: f64, t95: f64) -> (f64, f64, f64) {
    let t55997 = 192.0_f64 * t37441;
    let t56006 = 240.0_f64 * t28540;
    let t56008 = t55997 + 2.0_f64 * t1310 * t16301 + 2.0_f64 * t48024 - 14.0_f64 * t48028 + 140.0_f64 / 3.0_f64 * t28530 - t22290 + t22293 + t23336 + 0.77534644304710291488e-2_f64 * t95 * t606 * t55933 + t56006 - 14.0_f64 * t48040;
    (t55997, t56006, t56008)
}

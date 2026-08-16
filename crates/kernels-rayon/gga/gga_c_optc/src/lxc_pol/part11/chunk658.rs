//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 658/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk658(t3235: f64, t5355: f64, t3245: f64, t5318: f64, t914: f64, t5285: f64, t5289: f64, t1162: f64, t1179: f64, t1520: f64, t1536: f64, t3234: f64, t3244: f64, t4444: f64, t4450: f64, t4486: f64, t4489: f64, t4510: f64, t4513: f64, t5298: f64, t5302: f64, t5337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5356 = t3235 * t5355;
    let t5359 = t3245 * t5355;
    let t5364 = t914 * t5318;
    let t5375 = t914 * t5285;
    let t5378 = t914 * t5289;
    let t5385 = 0.779739765264702906e1_f64 * t3234 * t5356 + 0.75734008510040627574e0_f64 * t3244 * t5359 - 0.10076140891672839458e-1_f64 * t1179 * t5337 - 0.57954409931925052364e-1_f64 * t1162 * t5364 + 0.83967840763940328814e-2_f64 * t1179 * t5302 - 0.5373941808892181044e-1_f64 * t4444 * t1520 + 0.50380704458364197288e-2_f64 * t1179 * t5298 - 0.15454509315180013964e0_f64 * t4450 * t1536 + 0.28977204965962526182e-1_f64 * t1162 * t5375 + 0.38636273287950034909e-1_f64 * t1162 * t5378 + 0.6717427261115226305e-2_f64 * t4486 + 0.19318136643975017455e-1_f64 * t4489 + 0.779739765264702906e1_f64 * t4510 + 0.75734008510040627574e0_f64 * t4513;
    (t5356, t5359, t5364, t5375, t5378, t5385)
}

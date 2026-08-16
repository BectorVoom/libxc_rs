//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1328/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1328(t2919: f64, t8743: f64, t1102: f64, t8549: f64, t8769: f64, t26261: f64, t26264: f64, t26252: f64, t26258: f64, t26268: f64, t26271: f64, t26326: f64, t26328: f64, t26330: f64, t26332: f64, t26347: f64, t26351: f64, t26354: f64, t26358: f64) -> (f64, f64, f64) {
    let t26490 = 0.70178680769462448852e1_f64 * t8743 * t2919;
    let t26493 = 0.46785787179641632568e1_f64 * t1102 * t8549 * t8769;
    let t26496 = 0.31310740740740740741e1_f64 * t26261;
    let t26497 = 0.13490888888888888889e1_f64 * t26264;
    let t26508 = 0.44729629629629629629e0_f64 * t26252 + 0.40256666666666666666e1_f64 * t26258 + t26496 + t26497 + 0.6189328125e-1_f64 * t26268 + 0.247573125e0_f64 * t26271 + 0.258925e1_f64 * t26347 - 0.80513333333333333336e0_f64 * t26326 - 0.53675555555555555556e0_f64 * t26328 - 0.44152e0_f64 * t26351 + 0.44152e0_f64 * t26354 + 0.16102666666666666667e1_f64 * t26330 + 0.12524296296296296297e1_f64 * t26332 + 0.98115555555555555556e0_f64 * t26358;
    (t26490, t26493, t26508)
}

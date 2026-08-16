//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 998/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk998(t11997: f64, t11998: f64, t12011: f64, t12028: f64, t209: f64, t3699: f64, t501: f64, t605: f64, t1377: f64, t3718: f64, t1382: f64, t12007: f64, t549: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12030 = t11997 + t11998 + t12011 + t12028;
    let t12031 = t12030 * t209;
    let t12032 = t3699 * t501;
    let t12033 = t12032 * t605;
    let t12034 = t1377 * t3718;
    let t12035 = t3718 * t605;
    let t12036 = t1382 * t12035;
    let t12037 = 2.0_f64 * t12036;
    let t12038 = t549 * t12007;
    (t12030, t12031, t12032, t12033, t12034, t12035, t12036, t12037, t12038)
}
